use std::{fs, path::Path};



pub fn mutations_read_dir(dir: &str) -> std::io::Result<Vec<String>> {
	let mut mutations: Vec<String> = Vec::new();
	let mutations_path = Path::new(dir);

	for entry in fs::read_dir(mutations_path)? {
		let entry: fs::DirEntry      = entry?;
		let path: std::path::PathBuf = entry.path();
		//println!("> {:?}", path);
		mutations.push(path.display().to_string());

	}

	Ok(mutations)
}
