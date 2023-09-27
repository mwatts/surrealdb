use crate::key::error::KeyCategory;
use crate::key::key_req::KeyRequirements;
use derive::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Key)]
pub struct Us<'a> {
	__: u8,
	_a: u8,
	pub ns: &'a str,
	_b: u8,
	pub db: &'a str,
	_c: u8,
	_d: u8,
	_e: u8,
	pub user: &'a str,
}

pub fn new<'a>(ns: &'a str, db: &'a str, user: &'a str) -> Us<'a> {
	Us::new(ns, db, user)
}

pub fn prefix(ns: &str, db: &str) -> Vec<u8> {
	let mut k = super::all::new(ns, db).encode().unwrap();
	k.extend_from_slice(&[b'!', b'u', b's', 0x00]);
	k
}

pub fn suffix(ns: &str, db: &str) -> Vec<u8> {
	let mut k = super::all::new(ns, db).encode().unwrap();
	k.extend_from_slice(&[b'!', b'u', b's', 0xff]);
	k
}

impl KeyRequirements for Us<'_> {
	fn key_category(&self) -> KeyCategory {
		KeyCategory::DatabaseUser
	}
}

impl<'a> Us<'a> {
	pub fn new(ns: &'a str, db: &'a str, user: &'a str) -> Self {
		Self {
			__: b'/',
			_a: b'*',
			ns,
			_b: b'*',
			db,
			_c: b'!',
			_d: b'u',
			_e: b's',
			user,
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn key() {
		use super::*;
		#[rustfmt::skip]
		let val = Us::new(
			"testns",
			"testdb",
			"testuser",
		);
		let enc = Us::encode(&val).unwrap();
		assert_eq!(enc, b"/*testns\x00*testdb\x00!ustestuser\x00");
		let dec = Us::decode(&enc).unwrap();
		assert_eq!(val, dec);
	}

	#[test]
	fn test_prefix() {
		let val = super::prefix("testns", "testdb");
		assert_eq!(val, b"/*testns\0*testdb\0!us\0");
	}

	#[test]
	fn test_suffix() {
		let val = super::suffix("testns", "testdb");
		assert_eq!(val, b"/*testns\0*testdb\0!us\xff");
	}
}
