#[derive(Clone)]
pub struct AppState {
	pub db: sled::Db,
}

impl AppState {
	pub fn new(db: sled::Db) -> Self {
		Self { db: db.clone() }
	}
}
