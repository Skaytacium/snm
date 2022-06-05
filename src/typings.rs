use std::cmp::Ordering;
use std::path::PathBuf;

use crate::util::parse_version;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(untagged)]
pub enum Lts {
	Version(String),
	False(bool),
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Entry {
	pub version: String,
	pub lts: Lts,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Saved {
	pub current: String,
	pub available: Vec<Entry>,
}

pub struct Dir {
	pub home: PathBuf,
	pub os: String,
	pub ext: String,
}

impl Dir {
	pub fn bin(&self) -> PathBuf {
		self.home.join("bin")
	}

	pub fn saved(&self) -> PathBuf {
		self.home.join("saved")
	}

	pub fn version(&self, version: &str) -> PathBuf {
		self.home
			.join("versions")
			.join(format!("{}.{}", version, self.ext))
	}
}

impl EntryList for Vec<Entry> {
	fn get_latest_lts(&self) -> Option<&Entry> {
		for entry in self {
			if let Lts::Version(_) = entry.lts {
				return Some(entry);
			}
		}
		None
	}

	fn get_from_version(&self, version: &String) -> Option<&Entry> {
		for entry in self {
			if version == &entry.version {
				return Some(entry);
			}
		}
		None
	}
}

impl PartialEq for Entry {
	fn eq(&self, other: &Self) -> bool {
		self.version == other.version
	}
}

impl PartialOrd for Entry {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		let a = parse_version(&self.version);
		let b = parse_version(&other.version);

		for i in 0..=2 {
			if a[i] > b[i] {
				return Some(Ordering::Greater);
			} else if a[i] < b[i] {
				return Some(Ordering::Less);
			} else if a[i] == b[i] {
				if i == 2 {
					return Some(Ordering::Equal);
				}
			}
		}
		None
	}
}

impl Eq for Entry {}

impl Ord for Entry {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

pub trait EntryList {
	fn get_latest_lts(&self) -> Option<&Entry>;
	fn get_from_version(&self, version: &String) -> Option<&Entry>;
}
