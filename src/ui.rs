use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
	/// Version to operate on.
	version: Option<String>,

	/// Install a version
	#[clap(short, long, parse(from_flag))]
	install: u8
}