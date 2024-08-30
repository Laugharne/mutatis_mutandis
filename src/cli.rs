
use crate::{utils::shell_call, Globals};

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
	println!("path : {}", g.fwd);

	// get <project_name> from path (?!)
	let last_element = g.fwd.split('/').last().unwrap_or("");
	println!("project_name : {}", last_element);
	let cargo_toml = format!("{}/programs/{}/Cargo.toml", g.fwd, last_element);
	println!("Cargo.toml : {}", cargo_toml);

	// check Cargo.toml
	// check .anchor/
	// programs/<project_name>/src/

	// check .mutatis/
}

fn cli_name(g: &Globals) {
	println!("mutatis {}\n", g.args.get(1).unwrap());
}