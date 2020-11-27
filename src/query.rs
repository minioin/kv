use serde::Deserialize;

#[derive(Deserialize)]
#[serde(default)]
pub struct Query {
	pub tags: Vec<String>,
}

impl Default for Query {
	fn default() -> Self {
		Self { tags: vec![] }
	}
}
