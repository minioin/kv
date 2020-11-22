use std::io;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
	#[error("I/O Error")]
	IOError(#[from] io::Error),

	#[error("Sled Error")]
	DatabaseError(#[from] sled::Error),

	#[error("Serialization Error")]
	SerializationError(#[from] serde_cbor::Error),

	#[error("unknown data store error")]
	Unknown,
}
