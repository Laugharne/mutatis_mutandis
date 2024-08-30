use std::process;

use std::env;
use text_colorizer::*;
//use std::path::PathBuf;

pub mod cli;
use cli::*;//{cli_version,cli_versions, cli_help};
mod utils;

#[derive(Debug)]
#[derive(Clone)]
#[allow(dead_code)]
struct Globals {
	fwd : String,        // Full Workinbg Directory
	args: Vec<String>,   // cli arguments
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
		fwd : fwd,
		args: args,
	}

}




fn cli_options(g: &Globals) -> &Globals {

	let options = g.args.get(1);

	match g.args.get(1) {
		Some(option) => {
			let option = option.as_str();
			match option {
				"init"     => { cli_init(&g)},
				"analyze"  => {},
				"run"      => {},
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

	println!("\n");
	g
}


fn main() {
	let g: Globals = init_app();
	//println!("{:?}", g);
	let g = cli_options( &g);

	process::exit(0);
}

