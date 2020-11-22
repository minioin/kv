use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tide::{http::Mime, Response};
#[derive(Serialize, Deserialize)]
pub struct Entry {
	created: i64,
	updated: i64,
	data_type: String,
	data: Vec<u8>,
}

impl Entry {
	pub fn new(data: Vec<u8>, data_type: Mime) -> Self {
		let created = Utc::now().timestamp_millis();
		Self {
			created,
			updated: created,
			data_type: data_type.to_string(),
			data,
		}
	}

	pub fn update(&mut self, data: Vec<u8>) -> &Self {
		self.updated = Utc::now().timestamp_millis();
		self.data = data;
		self
	}

	pub fn bytes(&self) -> &Vec<u8> {
		&self.data
	}

	pub fn content_type(&self) -> Mime {
		Mime::from(self.data_type.as_str())
	}
}

impl From<Entry> for Response {
	fn from(data: Entry) -> Self {
		let created = DateTime::<Utc>::from_utc(
			NaiveDateTime::from_timestamp(data.created / 1000, 0),
			Utc,
		);
		let updated = DateTime::<Utc>::from_utc(
			NaiveDateTime::from_timestamp(data.updated / 1000, 0),
			Utc,
		);
		Response::builder(200)
			.content_type(data.content_type())
			.header("Created", created.to_rfc2822())
			.header("Last-Modified", updated.to_rfc2822())
			.body(data.data)
			.build()
	}
}
