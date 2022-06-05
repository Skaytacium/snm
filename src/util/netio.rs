use crate::typings::Dir;
use std::fs::File;

pub fn download<'a>(ver: &'a str, path: &Dir) -> Result<(), String> {
	let mut file = match File::create(path.version(ver)) {
		Ok(f) => f,
		Err(_) => return Err("couldn't open file for download".to_string()),
	};

	match ureq::get(
		&format!(
			"https://nodejs.org/dist/{}/node-{}-{}.{}",
			&ver, &ver, &path.os, &path.ext
		)[..],
	)
	.call()
	{
		Ok(resp) => match std::io::copy(&mut resp.into_reader(), &mut file) {
			Ok(_) => return Ok(()),
			Err(_) => return Err("couldn't write to file".to_string()),
		},
		Err(_) => return Err("couldn't download file".to_string()),
	};
}

pub fn remove<'a>(ver: &'a str, path: &Dir) -> Result<(), String> {
	match std::fs::remove_file(path.version(ver)) {
		Ok(_) => Ok(()),
		Err(_) => Err("can't delete file (not found/no permissions)".to_string()),
	}
}

pub fn use_version(ver: &str, path: &Dir) -> Result<(), String> {
	let file = match File::open(path.version(&ver)) {
		Ok(f) => f,
		Err(_) => return Err(format!("couldn't open {}", path.version(&ver).display())),
	};
	let mut archive = zip::ZipArchive::new(file).expect("zip error");

	for i in 0..archive.len() {
		let mut file = archive.by_index(i).unwrap();
		let outpath = path.bin();

		if (*file.name()).ends_with('/') {
			println!("File {} extracted to \"{}\"", i, outpath.display());
			std::fs::create_dir_all(&outpath).unwrap();
		} else {
			println!(
				"File {} extracted to \"{}\" ({} bytes)",
				i,
				outpath.display(),
				file.size()
			);
			if let Some(p) = outpath.parent() {
				if !p.exists() {
					std::fs::create_dir_all(&p).unwrap();
				}
			}
			let mut outfile = std::fs::File::create(&outpath).unwrap();
			std::io::copy(&mut file, &mut outfile).unwrap();
		}
	}

	Ok(())
}
