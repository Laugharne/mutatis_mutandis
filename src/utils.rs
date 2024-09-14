use std::process::Command;
use std::{fs, process};
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use text_colorizer::*;
use std::io;
use std::path::{Path, PathBuf};

pub const IDENT: &str = "  ";



pub fn shell_call(cmd: &str, args: &str) {
	let output = Command::new(cmd)
		.arg(args)
		.output()
		.expect("Fail to execute command");

	if output.status.success() {
		print!("{}", String::from_utf8_lossy(&output.stdout));
	} else {
		let msg = String::from_utf8_lossy(&output.stderr);
		eprintln!("{}Execution error :\n{}", IDENT, msg.red());
	}
}


pub fn check_file_exists(file: &str, message: &str) {
	if Path::new(file).exists() { return;}
	eprint_exit(message);
}


pub fn dir_exists(dir: &str, message: &str) {
	if Path::new(dir).is_dir() { return;}
	eprint_exit(message);
}

pub fn add_to_txt_file(file: &str, text: &str) {
	let mut content = read_to_string(&file).unwrap();
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.open(file).unwrap();

	let text_to_append = text;
	let _ = file.write_all(text_to_append.as_bytes());
}

pub fn check_and_add_to_txt_file(file: &str, text: &str) -> u8 {
	let mut content = read_to_string(&file).unwrap();
	if ! content.contains(text) {
		let path = Path::new(file);
		if let Some(file_name) = path.file_name() {
			println!("- `{:?}` : Add \"{}\"", file_name, text);
			add_to_txt_file(file, text);
			return 1;
		} else {
			eprint_exit("Path don't contains any file !");
		}
	}
	0
}


pub fn eprint_exit(message: &str) {
	eprintln!("{}{}", IDENT, message.red());
	process::exit(1);
}




pub fn clear_directory(dir: &str) -> std::io::Result<()> {
	// Itérer sur le contenu du répertoire
	let dir_path = Path::new(dir);

	for entry in fs::read_dir(dir_path)? {
		let entry: fs::DirEntry      = entry?;
		let path: std::path::PathBuf = entry.path();

		// Si c'est un répertoire, supprimer tout son contenu récursivement
		if path.is_dir() {
			fs::remove_dir_all(&path)?;
		} else {
			// Sinon, c'est un fichier, donc le supprimer
			fs::remove_file(&path)?;
		}
	}
	Ok(())
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
	// Créer le répertoire de destination s'il n'existe pas
	if !dst.exists() {
		fs::create_dir_all(dst)?;
	}

	// Itérer sur les entrées dans le répertoire source
	for entry in fs::read_dir(src)? {
		let entry = entry?;
		let entry_path = entry.path();
		let destination = dst.join(entry.file_name());

		if entry_path.is_dir() {
			// Copier récursivement les sous-répertoires
			copy_dir_all(&entry_path, &destination)?;
		} else {
			// Copier les fichiers
			fs::copy(&entry_path, &destination)?;
		}
	}
	Ok(())
}

struct SourceCode {
	path: PathBuf,
	done: bool,
	index: u16,
}


pub fn parse_directories(dir: &Path) -> io::Result<Vec<PathBuf>> {
	let mut files = Vec::new();

	// Read directory content
	for entry in fs::read_dir(dir)? {
		let entry = entry?;
		let path = entry.path();

		if path.is_dir() {
			files.extend(parse_directories(&path)?);
		} else {
			files.push(path);
		}
	}

	Ok(files)
}