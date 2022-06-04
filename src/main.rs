mod ui;
mod util;
use clap::Parser;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
enum Lts {
	Version(String),
	False(bool),
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Entry {
	version: String,
	lts: Lts,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Saved {
	current: String,
	available: Vec<Entry>,
}

impl PartialEq for Entry {
	fn eq(&self, other: &Self) -> bool {
		self.version == other.version
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// (OS, extension)
	let file = match std::env::consts::OS {
		"linux" => ("linux-x64", "tar.xz"),
		"windows" => ("win-x64", "zip"),
		"macos" => ("darwin-x64", "tar.xz"),
		_ => panic!("OS not supported"),
	};

	let mut path = match home::home_dir() {
		Some(mut dir) => {
			dir.push(".snm");
			dir
		}
		None => {
			println!("Could not find path to user home");
			return Ok(());
		}
	};

	let cli = ui::Cli::parse();

	if cli.path {
		path.push("bin");
		path.push("node");
		println!("{}", path.display());
		path.pop();
		path.pop();
	} else if let Some(input) = cli.remove {
		path.push("saved");
		let resp = util::get_saved(&path);
		path.pop();

		match util::parse_version(&input, &resp.available) {
			Ok(v) => println!("{}", v),
			Err(s) => {
				println!("{}", s);
				return Ok(());
			}
		}
	} else {
		let resp = reqwest::get("https://nodejs.org/dist/index.json")
			.await?
			.json::<Vec<Entry>>()
			.await?;

		if let Some(input) = cli.install {
			let resp = util::get_link(&util::parse_version(&input, &resp).unwrap(), &file);

			println!("{}", resp);
		} else if cli.list {
			path.push("saved");
			println!("{}", util::make_list(util::get_saved(&path), &resp).join("\n"));
			path.pop();
		}
	}

	Ok(())
}
