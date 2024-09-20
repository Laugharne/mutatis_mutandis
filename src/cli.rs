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
		&g,
		&format!("{}/{}", g.fwd, "Cargo.toml"),
		"Doesn't seems to be a Rust project !"
	);

	//-let file: String = format!("{}/{}", g.fwd, "Anchor.toml");
	check_file_exists(
		&g,
		&format!("{}/{}", g.fwd, "Anchor.toml"),
		"doesn't seems to ban Anchor project !"
	);

	//-let file: String = format!("{}/programs/{}/Cargo.toml", g.fwd, dir_project_name);
	check_file_exists(
		&g,
		&format!("{}/programs/{}/Cargo.toml", g.fwd, dir_project_name),
		"Cargo.toml doesn't exists in programs directory !"
	);

	let git_ignore: &str = ".gitignore";
	let git_ignore_full_path: String = format!("{}/{}", g.fwd, git_ignore);
	check_file_exists(&g, &git_ignore_full_path, "No `.gitignore` !?");

	// programs/<project_name>/src/
	let src_dir: String = format!("{}/programs/{}/src", g.fwd, dir_project_name);
	dir_exists(&g, &src_dir, "source directory 'src' doesn't exists !");

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
	nn += check_and_add_to_txt_file(&g, &git_ignore_full_path, "/.mutatis\n");
	nn += check_and_add_to_txt_file(&g, &git_ignore_full_path, "/test-ledger\n");

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
	check_file_exists(&g, &file, "Doesn't seems to have a `mutatis.toml` file !");

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
		(0..file.entry_point).for_each(|_entry| {
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



pub fn cli_run(mut g: &Globals) {

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

	let main_toml = main_toml_read(&g).unwrap();

	let path_to_validator: String = main_toml.mutation.validator_node;
	let path_of_execution: String = main_toml.mutation.test_ledger_path;
	let test_cmd: String          = main_toml.mutation.test_cmd;
	let validator_pause: u8       = main_toml.mutation.validator_pause;
	// println!("{:?}", path_to_validator);
	// println!("{:?}", path_of_execution);
	// println!("{:?}", test_cmd);
	// println!("{:?}", validator_pause);
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

	let mut mutation_success: u16 = 0;
	let mut mutation_fail: u16    = 0;

	//println!("- {}", mutations_path.display());
	//println!(":: {:?}", m);
	for mutation in mutations {
		//println!("> {}", mutation);
		let mutation_dir_name: &str = mutation.split('/').last().unwrap_or("");
		println!("{}{} {}", IDENT, "-".red(), mutation_dir_name);
		let mutation_toml: String   = format!("{}/{}.toml", mutation, mutation_dir_name);
		//println!("> {}", mutation_toml);
		let mutation_current = mutation_toml_read( &g, &mutation_toml).unwrap();

		let mut processus = validator_lanch(
			&path_to_validator,
			&path_of_execution,
			validator_pause,
		).unwrap();

		let src_file: String          = mutation_current.general.full_file_path;
		let mutated_file_name: &str   = src_file.split('/').last().unwrap_or("");
		let mutated_full_path: String = format!("{}{}/{}", mutations_dir, mutation_dir_name, mutated_file_name);
		let log_full_path: String     = format!("{}{}/log.txt", mutations_dir, mutation_dir_name);
		let backup_full_path: String  = format!("{}{}/backup/{}", mutations_dir, mutation_dir_name, mutated_file_name);
		// println!("> {}\n", src_file);
		// println!("> {}\n", mutated_full_path);
		// println!("> {}\n", backup_full_path);
		// println!("> {}\n", log_full_path);

		// copy mutated file --> src
		let _ = fs::copy(
			Path::new(&mutated_full_path),
			Path::new(&src_file)
		);

		// println!("Appuyez sur Entrée ...");
		// let _ = io::stdout().flush(); // Assurez-vous que le message est affiché avant d'attendre
		// let mut buffer = String::new();
		// io::stdin().read_line(&mut buffer);

		// tests --> log
		if anchor_tests(&g, &test_cmd, &log_full_path) == true {
			mutation_success += 1;
		} else {
			mutation_fail += 1;
		}

		// restore original file --> src
		let _ = fs::copy(
			Path::new(&backup_full_path),
			Path::new(&src_file)
		);

		let _ = validator_stop(processus);
		println!("");

	}

	if mutation_success > 0 {
		println!("{}✅ {}: {}", IDENT, "Success".green(), mutation_success);
	}

	if mutation_fail > 0 {
		println!("{}❌ {}: {}", IDENT, "Fail".red(), mutation_fail);
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