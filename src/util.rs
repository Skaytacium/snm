pub mod init;
pub mod netio;
pub mod saved;

use crate::typings;
use typings::EntryList;

pub fn get_version<'a>(
	input: &String,
	response: &'a Vec<typings::Entry>,
) -> Result<&'a String, &'static str> {
	match &input[..] {
		"latest" => Ok(&response.get(0).unwrap().version),
		"lts" => match &response.get_latest_lts() {
			Some(v) => Ok(&v.version),
			None => Err("no LTS version available"),
		},
		_ => {
			let version = if &input[..1] != "v" {
				format!("v{}", &input)
			} else {
				input.clone()
			};

			for entry in response {
				if entry.version[..].starts_with(&version[..]) {
					return Ok(&entry.version);
				}
			}
			return Err("version not available");
		}
	}
}

pub fn make_list(
	saved: typings::Saved,
	response: &Vec<typings::Entry>,
	dir: &typings::Dir,
) -> Vec<String> {
	let mut list: Vec<String> = Vec::new();

	for i in 0..saved.available.len() {
		let e = saved.available.get(i).unwrap();
		list.push(String::new());

		if e.version == saved.current {
			list[i] = format!("* {}", e.version);
		} else {
			list[i] = format!("- {}", e.version);
		}

		if e.version == response.get(0).unwrap().version {
			list[i].push_str(" (latest)");
		}

		if let Some(v) = &response.get_latest_lts() {
			if e.version == v.version {
				list[i].push_str(" (lts)")
			}
		}

		if dir.version(&e.version, false).exists() {
			list[i].push_str(" (installed)")
		}
	}

	list
}

pub fn parse_version(version: &String) -> [u8; 3] {
	let temp: Vec<&str> = version[1..].split(".").collect();

	[
		temp[0].parse().unwrap(),
		temp[1].parse().unwrap(),
		temp[2].parse().unwrap(),
	]
}
