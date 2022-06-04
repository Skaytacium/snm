use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
	/// View installed path
	#[clap(short, long)]
	pub path: bool,

	/// View installed versions
	#[clap(short, long)]
	pub list: bool,

	/// Install a version
	#[clap(short, long)]
	pub install: Option<String>,

	/// Remove a version
	#[clap(short, long)]
	pub remove: Option<String>,
}
