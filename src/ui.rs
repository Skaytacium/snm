use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
	/// use this version
	pub version: Option<String>,

	/// view installed path
	#[clap(short, long)]
	pub path: bool,

	/// view installed versions
	#[clap(short, long)]
	pub list: bool,

	/// install a version
	#[clap(short, long)]
	pub install: Option<String>,

	/// remove a version
	#[clap(short, long)]
	pub remove: Option<String>,
}
