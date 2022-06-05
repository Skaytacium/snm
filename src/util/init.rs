use std::io::Write;

pub fn make_default_config(path: &crate::typings::Dir) -> Result<(), std::io::Error> {
	make_directories(&path)?;
	let mut file = std::fs::File::create(&path.saved())?;
	file.write_all(b"{\"current\": \"\", \"available\": []}")?;
	Ok(())
}

fn make_directories(path: &crate::typings::Dir) -> Result<(), std::io::Error> {
	std::fs::create_dir_all(path.home.join("bin"))?;
	std::fs::create_dir(path.home.join("versions"))?;
	Ok(())
}
