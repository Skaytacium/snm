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
				.expect("could not create required directories/files");
			get_saved(&path)
		}
	}
}

pub fn save(path: &crate::typings::Dir, saved: &typings::Saved) {
	let mut file = File::create(path.saved()).expect("could not open ~/.snm/saved in write more");

	file.write(
		&serde_json::to_string(saved)
			.expect("couldn't convert to json")
			.as_bytes(),
	)
	.expect("could not write to ~/.snm/saved");
}
