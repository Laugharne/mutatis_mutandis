/// The line `use std::process;` in Rust is bringing the `process` module from the standard library into
/// the current scope. This allows you to use functions and types defined in the `process` module
/// without having to prefix them with `std::process::`.
use std::process;

use std::env;
use std::process::Child;
use text_colorizer::*;
//use std::path::PathBuf;

mod cli;
mod toml;
mod utils;
mod default;
mod analyze;
mod mutation;

mod pass1;
mod pass2;

use cli::*;//{cli_version,cli_versions, cli_help};

#[derive(Debug)]
#[derive(Clone)]
#[allow(dead_code)]
struct Globals {
	fwd           : String,        // Full Working Directory
	args          : Vec<String>,   // cli arguments
	//validator_flag: bool,          // Validator node launched (yes if true)
	//validator_child: Child,
}


/// The `init_app` function initializes the application by parsing command line arguments and setting up
/// global variables.
///
/// Returns:
///
/// The function `init_app()` returns a `Globals` struct.
fn init_app() -> Globals {
	// TODO intercept ctrl-c to stop processing and write results on output file !

	let args: Vec<String> = env::args().collect();
	let suffix = "mutatis_mutandis";

	let arg0 = args.get(0).unwrap();

	if !arg0.ends_with(suffix) {
		eprintln!("Unknow executable '{}'.", arg0.red());
		process::exit(1);
	}

	let mut fwd: String = "".to_string();

	match env::current_dir() {
		Ok(path) => {
			fwd = path.display().to_string();
		},
		Err(e) => {
			eprintln!("Impossible to get current working directory: {}", e);
			process::exit(1);
		},
	}

	//-println!("{:?}", args);

	Globals {
		fwd      : fwd,
		args     : args,
		//validator_flag: false,
	}

}




fn cli_options(g: &Globals) -> &Globals {

	let options = g.args.get(1);

	// dispatcher
	match g.args.get(1) {
		Some(option) => {
			let option = option.as_str();
			match option {
				"init"     => { cli_init(&g)},
				"analyze"  => { cli_analyze(&g)},
				"run"      => { cli_run(&g)},
				"reset"    => {},
				"clear"    => {},
				"remove"   => {},
				"help"     => { cli_help();},
				"version"  => { cli_version()},
				"versions" => { cli_versions()},
				_          => {
					eprintln!("Unknow option '{}'", option);
					process::exit(1);
				}
			}
		},
		None => {
			cli_help();
			eprintln!("No options !?");
			process::exit(1);
		}

	}

	//println!("\n");
	g
}


fn main() {
	let g: Globals = init_app();
	//println!("{:?}", g);
	let _g = cli_options( &g);

	process::exit(0);
}

