//use std::process;
use std::path::Path;
use crate::{utils::*, Globals};
use std::fs::{OpenOptions, read_to_string};

/// Displays the command-line interface (CLI) help information for the mutatis application.
///
/// This function provides guidance on the usage of the mutatis tool, including examples
/// of how to use it with various command-line arguments.
pub fn cli_help() {
	// TODO
}

pub fn cli_version() {
	println!("mutatis {}", env!("CARGO_PKG_VERSION"));
}

pub fn cli_versions() {
	cli_version();
	// TODO
	shell_call("rustc", "--version");
	shell_call("cargo", "--version");
	shell_call("solana", "-V");
	shell_call("anchor", "-V");
}

pub fn cli_init(g: &Globals) {
	cli_name(&g);
	//println!("path : {}", g.fwd);

	// get <project_name> from path (?!)
	let dir_project_name: &str = g.fwd.split('/').last().unwrap_or("");

	// check for some important files !
	let file: String = format!("{}/{}", g.fwd, "Cargo.toml");
	check_file_exists(&file, "Doesn't seems to be a Rust project !");
	let file = format!("{}/{}", g.fwd, "Anchor.toml");
	check_file_exists(&file, "doesn't seems to ban Anchor project !");
	let file: String = format!("{}/programs/{}/Cargo.toml", g.fwd, dir_project_name);
	check_file_exists(&file, "Cargo.toml doesn't exists in programs directory !");

	let git_ignore: String = format!("{}/{}", g.fwd, ".gitignore");
	check_file_exists(&git_ignore, "No `.gitignore` !?");

	// programs/<project_name>/src/
	let src_dir: String = format!("{}/programs/{}/src", g.fwd, dir_project_name);
	dir_exists(&src_dir, "source directory 'src' doesn't exists !");

	// check .mutatis/
	let mutatis_dir: String = format!("{}/.mutatis", g.fwd);
	if Path::new(&mutatis_dir).is_dir() {
		//println!("Le répertoire existe !");
	} else {
		shell_call("mkdir", &mutatis_dir);
		let backup_dir: String = format!("{}/.mutatis/backup", g.fwd);
		shell_call("mkdir", &backup_dir);
		let logs_dir: String = format!("{}/.mutatis/logs", g.fwd);
		shell_call("mkdir", &logs_dir);
		let tmp_dir: String = format!("{}/.mutatis/tmp", g.fwd);
		shell_call("mkdir", &tmp_dir);
		let mutations_dir: String = format!("{}/.mutatis/mutations", g.fwd);
		shell_call("mkdir", &mutations_dir);
	}

	// `.gitignore`
	// sanity check, add '\n' if needed
	let mut content = read_to_string(&git_ignore).unwrap();
	if !content.ends_with('\n') {
		add_to_txt_file(&git_ignore, "\n");
	}

	// add content in `.gitignore`
	let mut nn: u8 = 0;
	nn += check_and_add_to_txt_file(&git_ignore, "/.mutatis\n");
	nn += check_and_add_to_txt_file(&git_ignore, "/test-ledger\n");

	

}

fn cli_name(g: &Globals) {
	println!("mutatis {}\n", g.args.get(1).unwrap());
}