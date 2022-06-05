use crate::typings;
use std::fs::File;
use std::io::{Read, Write};

pub fn get_saved(path: &crate::typings::Dir) -> typings::Saved {
	let file = File::open(&path.saved());

	match file {
		Ok(mut f) => {
			let mut file_contents = String::new();
			f.read_to_string(&mut file_contents)
				.expect("couldn't read from config file");

			match serde_json::from_str::<typings::Saved>(&file_contents) {
				Ok(mut o) => {
					o.available.sort();
					o.available.reverse();
					o
				}
				Err(_) => panic!("saved data corrupt..."),
			}
		}
		Err(_) => {
			super::init::make_default_config(&path)
				.expect("couldn't create required directories/files");
			get_saved(&path)
		}
	}
}

pub fn save(path: &crate::typings::Dir, saved: &typings::Saved) -> Result<(), String> {
	if get_saved(&path).current != saved.current {
		super::netio::use_version(&saved.current, &path)?;
	}

	let mut file = match File::create(path.saved()) {
		Ok(f) => f,
		Err(_) => return Err("couldn't open ~/.snm/saved in write more".to_string()),
	};

	match file.write(
		&serde_json::to_string(saved)
			.expect("couldn't convert to json")
			.as_bytes(),
	) {
		Ok(_) => return Ok(()),
		Err(_) => return Err("couldn't write to file".to_string()),
	}
}
