use crate::typings::Dir;
use std::{fs, path};

pub fn download<'a>(ver: &'a str, path: &Dir) -> Result<(), &'static str> {
	let mut file = match fs::File::create(path.version(ver, true)) {
		Ok(f) => f,
		Err(_) => return Err("couldn't open file for download"),
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
			Err(_) => return Err("couldn't write to file"),
		},
		Err(_) => return Err("couldn't download file"),
	};
}

pub fn remove<'a>(ver: &'a str, path: &Dir) -> Result<(), &'static str> {
	let mut err_cnt: u8 = 0;

	if path.version(ver, true).exists() {
		if let Err(_) = std::fs::remove_file(path.version(ver, true)) {
			err_cnt += 1;
		}
	}
	if path.version(ver, false).exists() {
		if let Err(_) = std::fs::remove_dir_all(path.version(ver, false)) {
			err_cnt += 1;
		}
	}
	if err_cnt > 1 {
		return Err("could not remove version (not installed)");
	}

	Ok(())
}

pub fn use_version(ver: &str, path: &Dir) -> Result<(), &'static str> {
	if !path.version(ver, false).exists() {
		println!("installing {}...", &ver);

		let file = match fs::File::open(path.version(&ver, true)) {
			Ok(f) => f,
			Err(_) => return Err("couldn't open file for extraction"),
		};

		{
			let mut archive = zip::ZipArchive::new(file).expect("zip error");

			for i in 0..archive.len() {
				let mut file = archive.by_index(i).unwrap();
				let outpath = path
					.version(&ver, false)
					.join(file.enclosed_name().unwrap());

				if (*file.name()).ends_with('/') {
					fs::create_dir_all(&outpath).unwrap();
				} else {
					if let Some(p) = outpath.parent() {
						if !p.exists() {
							fs::create_dir_all(&p).unwrap();
						}
					}

					let mut outfile = fs::File::create(&outpath).unwrap();
					std::io::copy(&mut file, &mut outfile).unwrap();
				}
			}
		}

		let outdir = &path
			.version(&ver, false)
			.join(format!("node-{}-{}", &ver, &path.os));

		recurse_dir(outdir, &path.version(&ver, false), false)?;

		// needs to delete folders
		if let Err(_) = fs::remove_dir_all(outdir) {
			return Err("couldn't delete useless directory");
		}
		if let Err(_) = fs::remove_file(&path.version(&ver, true)) {
			return Err("couldn't delete (now useless) archive");
		}
	}

	recurse_dir(&path.version(&ver, false), &path.home.join("bin"))?;

	println!("using {}", &ver);

	Ok(())
}

pub fn recurse_dir(
	from: &path::PathBuf,
	to: &path::PathBuf,
	symlink: bool,
) -> Result<(), &'static str> {
	if from.is_dir() && to.is_dir() {
		match fs::read_dir(from) {
			Ok(d) => {
				for e in d {
					let f = e.unwrap().path();
					let t = to.join(path::Path::new(&f.components().last().unwrap()));

					if f.is_dir() {
						if !t.exists() {
							if let Err(_) = fs::create_dir(&t) {
								return Err("couldn't create recursive directory");
							}
						}
						recurse_dir(&f, &t, symlink)?
					} else {
						match fs::rename(f, t) {
							Ok(_) => (),
							Err(_) => return Err("couldn't recursively move file"),
						};
					}
				}
			}
			Err(_) => return Err("couldn't read directory (not found)"),
		}
	} else {
		return Err("not a directory to copy from or/and to");
	}

	Ok(())
}
