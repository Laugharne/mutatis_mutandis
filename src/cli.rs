use std::{fs, path::{Path, PathBuf}};
use crate::{
	utils::*,
	toml::*,
	default::*,
	analyze::*,
	mutation::*,
	pass1::*,
	pass2::*,
	Globals
};
use std::fs::read_to_string;

use text_colorizer::*;

use std::io::{self, Write};

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
	//-let file: String = format!("{}/{}", g.fwd, "Cargo.toml");
	check_file_exists(
		&format!("{}/{}", g.fwd, "Cargo.toml"),
		"Doesn't seems to be a Rust project !"
	);

	//-let file: String = format!("{}/{}", g.fwd, "Anchor.toml");
	check_file_exists(
		&format!("{}/{}", g.fwd, "Anchor.toml"),
		"doesn't seems to ban Anchor project !"
	);

	//-let file: String = format!("{}/programs/{}/Cargo.toml", g.fwd, dir_project_name);
	check_file_exists(
		&format!("{}/programs/{}/Cargo.toml", g.fwd, dir_project_name),
		"Cargo.toml doesn't exists in programs directory !"
	);

	let git_ignore: &str = ".gitignore";
	let git_ignore_full_path: String = format!("{}/{}", g.fwd, git_ignore);
	check_file_exists(&git_ignore_full_path, "No `.gitignore` !?");

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
	let content: String = read_to_string(&git_ignore_full_path).unwrap();
	if !content.ends_with('\n') {
		add_to_txt_file(&git_ignore_full_path, "\n");
	}

	// add content in `.gitignore`
	let mut nn: u8 = 0;
	nn += check_and_add_to_txt_file(&git_ignore_full_path, "/.mutatis\n");
	nn += check_and_add_to_txt_file(&git_ignore_full_path, "/test-ledger\n");

	if nn > 0 {
		println!("\nAdded {} new lines to `.gitignore`", nn);
	}

	println!("");

	let test_cmd: String = qa(
		"Anchor test command",
		DEFAULT_TEST_CMD,
	);

	let validator_node: String = qa(
		"Validator node",
		DEFAULT_VALIDATOR_NODE,
	);

	let mutation_level: String = qa(
		"Mutation level",
		&DEFAULT_MUTATION_LEVEL.to_string(),
	);

	// 	let dir_project_name: &str = g.fwd.split('/').last().unwrap_or("");
	let ml: MutationLevel = mutation_level.parse::<u8>().unwrap_or_else(|_err| {
		eprintln!("{}{}", IDENT, "Conversion Error, level set to default !".red());
		1
	});


	let _ = main_toml_generation(
		&g,
		&test_cmd,
		&validator_node,
		ml
	);

	// println!("{}", test_cmd);
	// println!("{}", validator_node);
	// println!("{}", ml);

}


pub fn cli_analyze(g: &Globals) {
	//cli_name(&g);
	//println!("path : {}", g.fwd);
	let file: String = format!("{}/.mutatis/{}", g.fwd, "mutatis.toml");
	check_file_exists(&file, "Doesn't seems to have a `mutatis.toml` file !");

	// clean mutations !
	let mutations_dir: String = format!("{}/.mutatis/mutations/", g.fwd);
	//println!("{}", mutations_dir);

	match clear_directory(&mutations_dir) {
		Ok(_) => {},
		Err(_e) => {eprint_exit("Error occured during file erasing !");}
	}

	let dir_project_name: &str = g.fwd.split('/').last().unwrap_or("");
	let src_dir: String        = format!("{}/programs/{}/src", g.fwd, dir_project_name);

	let mut files: Vec<SourceCode> = parse_directories(&Path::new(&src_dir)).unwrap();

	let display: String = format!("Files to analyze: {:?}", files.len());
	println!("\n{}{}", IDENT, display.green());
	for file in &files {
		println!("{}{}{} {}", IDENT, IDENT, "-".red(), file.path_src_root);
	}

	pass1( &g, &src_dir, &mut files);

	let mut nn_mutation: IndexMutation = 0;

	let display: String = format!("Mutation entry point per file:");
	println!("\n{}{}", IDENT, display.green());
	for file in &files {
		println!("{}{}{} (x{})\t{}", IDENT, IDENT, "-".red(), file.entry_point, file.path_src_root);
		nn_mutation += file.entry_point;
	}

	pass2( &g, &src_dir, &mut files);

	let display: String = format!("Mutation generated: {}", nn_mutation);
	println!("\n{}{}", IDENT, display.green());
	let mut idx: IndexMutation = 0;
	for file in &files {
		println!("{}{}{} {}", IDENT, IDENT, "-".red(), file.path_src_root);
		(0..file.entry_point).for_each(|entry| {
			println!(
				"{}{}{}{} {}",
				IDENT, IDENT, IDENT,
				"-".red(),
				build_mutation_index_str(idx)
			);
			idx += 1;
		});
	}

}



pub fn cli_run(g: &Globals) {

	println!("");

	let commit: String = qa(
		"Have you proceed to a commit before ? (y/n)",
		DEFAULT_COMMIT_QUESTION,
	);

	if !commit.eq_ignore_ascii_case(DEFAULT_COMMIT_QUESTION) {
		println!("{}{}", IDENT, "Processus cancelled.".red());
		return;
	}

	//clean backup !
	let backup_dir: String = format!("{}/.mutatis/backup/", g.fwd);
	//println!("- {}", backup_dir);

	match clear_directory(&backup_dir) {
		Ok(_) => {},
		Err(_e) => {eprint_exit("Error occured during file erasing !");}
	}

	let _ = main_toml_read(&g);

	// get <project_name> from path (?!)
	let dir_project_name: &str = g.fwd.split('/').last().unwrap_or("");

	// programs/<project_name>/src/
	let src_dir: String = format!("{}/programs/{}/src", g.fwd, dir_project_name);
	//println!("- {}", src_dir);

	let _ = copy_dir_all(
		Path::new(&src_dir),
		Path::new(&backup_dir)
	);

	let mutations_dir: String  = format!("{}/.mutatis/mutations/", g.fwd);
	let mutations: Vec<String> = mutations_read_dir(&mutations_dir).unwrap();

	//println!("- {}", mutations_path.display());
	//println!(":: {:?}", m);
	for mutation in mutations {
		//println!("> {}", mutation);
		let mutation_dir_name: &str = mutation.split('/').last().unwrap_or("");
		let mutation_toml: String   = format!("{}/{}.toml", mutation, mutation_dir_name);
		//println!("> {}", mutation_toml);
		let _ = mutation_toml_read( &g, &mutation_toml);

	}

}


fn qa(question: &str, default: &str) -> String {
	let d: ColoredString = format!("{}Default: {}", IDENT, default).bright_black();
	println!("{}", d);

	let q: ColoredString = format!("{}> {}? ", IDENT, question).green();
	print!("{}", q);

	io::stdout().flush().unwrap();
	let mut user_intput: String = String::new();
	io::stdin()
		.read_line(&mut user_intput)
		.expect("Error reading user input.");
	println!("");

	let user_intput: String = user_intput.trim().to_string();
	if user_intput.is_empty() {
		return default.to_owned();
	}

	user_intput
}

fn cli_name(g: &Globals) {
	println!("mutatis {}", g.args.get(1).unwrap());
}