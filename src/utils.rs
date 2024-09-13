use std::process::Command;
use std::process;
use std::path::Path;
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;


pub const IDENT: &str = "  ";



pub fn shell_call(cmd: &str, args: &str) {
	let output = Command::new(cmd)
		.arg(args)
		.output()
		.expect("Fail to execute command");

	if output.status.success() {
		print!("{}", String::from_utf8_lossy(&output.stdout));
	} else {
		eprintln!("Execution error :\n{}", String::from_utf8_lossy(&output.stderr));
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
	eprintln!("{}", message);
	process::exit(1);
}