use kv::{
	appstate::AppState,
	entry::Entry,
	error::{Error, Result},
	query::Query,
};
use std::{path::PathBuf, str::FromStr};
use tide::{http::Mime, Request, StatusCode};

#[async_std::main]
async fn main() -> Result<()> {
	let path: PathBuf = dirs::data_dir()
		.unwrap_or_else(|| "/data".into())
		.join("kv");
	let db = sled::open(path.join("sled"))?;
	let app_state = AppState::new(db);
	let mut app = tide::with_state(app_state);
	tide::log::start();

	app.at("/kv/:bucket/:key")
		.get(get_kv)
		.post(put_kv)
		.put(put_kv)
		.patch(put_kv)
		.delete(delete_kv);
	app.listen("0.0.0.0:8000").await?;
	Ok(())
}

async fn get_kv(req: Request<AppState>) -> tide::Result {
	let bucket = req.param("bucket")?;
	let key = req.param("key")?;
	let content = req
		.state()
		.db
		.open_tree(bucket)
		.map_err(Error::from)?
		.get(key)
		.map_err(Error::from)?
		.unwrap();
	let data: Entry = serde_cbor::from_slice(&content).expect("couldn't parse");
	Ok(data.into())
}

async fn put_kv(mut req: Request<AppState>) -> tide::Result {
	let bucket = req.param("bucket")?.to_owned();
	let key = req.param("key")?.to_owned();
	let query: Query = req.query()?;
	let tags = query.tags;
	let content_type = req
		.content_type()
		.unwrap_or_else(|| Mime::from_str("application/octet-stream").unwrap());

	let tree = req.state().db.open_tree(bucket).map_err(Error::from)?;
	let old_content = tree.get(&key).map_err(Error::from)?;
	let new_content = req.body_bytes().await?;
	let entry = match old_content {
		Some(content) => {
			let mut old_entry: Entry =
				serde_cbor::from_slice(&content).expect("couldn't parse");
			if old_entry.content_type() != content_type {
				return Ok(StatusCode::BadRequest.into());
			}

			old_entry.update(new_content);
			old_entry
		},
		None => Entry::new(new_content, content_type, tags),
	};
	let value = serde_cbor::to_vec(&entry).map_err(Error::from)?;
	tree.insert(key, value).map_err(Error::from)?;
	Ok(StatusCode::Ok.into())
}

async fn delete_kv(req: Request<AppState>) -> tide::Result {
	let bucket = req.param("bucket")?;
	let key = req.param("key")?;
	req.state()
		.db
		.open_tree(bucket)
		.map_err(Error::from)?
		.remove(key)
		.map_err(Error::from)?;
	Ok(StatusCode::Ok.into())
}
