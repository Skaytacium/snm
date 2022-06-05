#[cfg(test)]
mod tests;
mod typings;
mod ui;
mod util;

use clap::Parser;

use crate::typings::EntryList;

fn main() {
	// (OS, extension)
	let file = match std::env::consts::OS {
		"linux" => ("linux-x64", "tar.xz"),
		"windows" => ("win-x64", "zip"),
		"macos" => ("darwin-x64", "tar.xz"),
		_ => {
			println!("OS not supported");
			return;
		}
	};

	let path = match home::home_dir() {
		Some(dir) => typings::Dir {
			home: dir.join(".snm"),
			os: file.0.to_string(),
			ext: file.1.to_string(),
		},
		None => {
			println!("user home not found");
			return;
		}
	};

	drop(file);

	let cli = ui::Cli::parse();
	let mut saved = util::saved::get_saved(&path);

	if cli.path {
		println!("{}", path.bin().join("node").display());
	} else if let Some(input) = cli.remove {
		match util::get_version(&input, &saved.available) {
			Ok(v) => {
				println!("removing {}...", &v);
				match util::netio::remove(&v, &path) {
					Ok(_) => (),
					Err(e) => {
						println!("{}", e);
						return;
					}
				}

				let pos = saved
					.available
					.iter()
					.position(|x| &x.version == v)
					.unwrap();

				if &saved.current == v {
					saved.available.remove(pos);
					saved.current = match saved.available.get(0) {
						Some(v) => v.version.clone(),
						None => "".to_string(),
					}
				} else {
					saved.available.remove(pos);
				}

				match util::saved::save(&path, &saved) {
					Ok(_) => (),
					Err(e) => {
						println!("{}", e);
						return;
					}
				};
			}
			Err(e) => println!("{}", e),
		}
	} else if let Some(input) = cli.version {
		match util::get_version(&input, &saved.available) {
			Ok(v) => {
				println!("using {}", v);

				saved.current = v.clone();
				match util::saved::save(&path, &saved) {
					Ok(_) => (),
					Err(e) => {
						println!("{}", e);
						return;
					}
				};
			}
			Err(e) => println!("{}", e),
		}
	} else {
		let resp = serde_json::from_str::<Vec<typings::Entry>>(
			&ureq::get("https://nodejs.org/dist/index.json")
				.call()
				.expect("couldn't send request")
				.into_string()
				.expect("couldn't decode response")[..],
		)
		.expect("couldn't parse json");

		if let Some(input) = cli.install {
			match util::get_version(&input, &resp) {
				Ok(v) => {
					let entry = resp.get_from_version(v).unwrap();

					if let Some(_) = saved.available.get_from_version(v) {
						println!("{} already installed", &v);
						return;
					}

					println!("installing {}...", &v);
					match util::netio::download(v, &path) {
						Ok(_) => (),
						Err(e) => {
							println!("{}", e);
							return;
						}
					};

					if &saved.current[..] == "" || entry > saved.available.get(0).unwrap() {
						saved.current = entry.version.clone()
					}
					saved.available.push(entry.clone());

					match util::saved::save(&path, &saved) {
						Ok(_) => (),
						Err(e) => {
							println!("{}", e);
							return;
						}
					};
				}
				Err(e) => println!("{}", e),
			}
		} else if cli.list {
			println!(
				"{}",
				util::make_list(util::saved::get_saved(&path), &resp).join("\n")
			);
		}
	}
}
