use std::{
	fs::File,
	io::{Read, Write},
};

pub fn parse_version<'a>(
	input: &String,
	response: &'a Vec<crate::Entry>,
) -> Result<&'a String, String> {
	match &input[..] {
		"latest" => Ok(&response.get(0).unwrap().version),
		"lts" => {
			for entry in response {
				if let crate::Lts::Version(_) = entry.lts {
					return Ok(&entry.version);
				};
			}
			return Err("No LTS versions available".to_string());
		}
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
			return Err("Specified version not found".to_string());
		}
	}
}

pub fn get_link(version: &String, file_tuple: &(&str, &str)) -> String {
	format!(
		"https://nodejs.org/dist/{}/node-{}-{}.{}",
		&version, &version, &file_tuple.0, &file_tuple.1
	)
}

pub fn get_saved(path: &std::path::PathBuf) -> crate::Saved {
	let file = File::open(&path);

	match file {
		Ok(mut f) => {
			let mut file_contents = String::new();
			f.read_to_string(&mut file_contents).unwrap();

			match serde_json::from_str(&file_contents) {
				Ok(o) => return o,
				Err(_) => {
					panic!("Saved data corrupt...");
				}
			}
		}
		Err(_) => {
			let mut file = File::create(&path).unwrap();
			file.write_all(b"{\"current\": \"\", \"available\": []}")
				.unwrap();

			return get_saved(path);
		}
	}
}

pub fn make_list(saved: crate::Saved, response: &Vec<crate::Entry>) -> Vec<String> {
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

		for entry in response {
			if let crate::Lts::Version(_) = entry.lts {
				if entry.version == e.version {
					list[i].push_str(" (lts)");
				}
				break;
			};
		}
	}

	list
}
