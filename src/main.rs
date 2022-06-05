#[cfg(test)]
mod tests;
mod typings;
mod ui;
mod util;

use clap::Parser;

use crate::typings::EntryList;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// (OS, extension)
	let file = match std::env::consts::OS {
		"linux" => ("linux-x64", "tar.xz"),
		"windows" => ("win-x64", "zip"),
		"macos" => ("darwin-x64", "tar.xz"),
		_ => {
			println!("OS not supported");
			return Ok(());
		}
	};

	let path = match home::home_dir() {
		Some(dir) => typings::Dir {
			home: dir.join(".snm"),
		},
		None => {
			println!("user home not found");
			return Ok(());
		}
	};

	let cli = ui::Cli::parse();

	let mut saved = util::saved::get_saved(&path);

	if cli.path {
		println!("{}", path.bin().display());
	} else if let Some(input) = cli.remove {
		match util::get_version(&input, &saved.available) {
			Ok(v) => {
				println!("removing {}...", &v);

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

				// TODO: implement removing a version

				util::saved::save(&path, &saved);
			}
			Err(e) => println!("{}", e),
		}
	} else if let Some(input) = cli.version {
		match util::get_version(&input, &saved.available) {
			Ok(v) => {
				println!("using {}", v);

				// TODO: implement changing the version

				saved.current = v.clone();
				util::saved::save(&path, &saved);
			}
			Err(e) => println!("{}", e),
		}
	} else {
		let resp = reqwest::get("https://nodejs.org/dist/index.json")
			.await?
			.json::<Vec<typings::Entry>>()
			.await?;

		if let Some(input) = cli.install {
			match util::get_version(&input, &resp) {
				Ok(v) => {
					let entry = resp.get_from_version(v).unwrap();

					if let Some(_) = saved.available.get_from_version(v) {
						println!("{} already installed", &v);
						return Ok(());
					}

					println!("installing {}...", &v);

					let link = format!(
						"https://nodejs.org/dist/{}/node-{}-{}.{}",
						&v, &v, &file.0, &file.1
					); //TODO: implement actually downloading the file
					println!("{}", link);

					if &saved.current[..] == "" || entry > saved.available.get(0).unwrap() {
						saved.current = entry.version.clone()
					}
					saved.available.push(entry.clone());

					util::saved::save(&path, &saved);
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

	Ok(())
}
