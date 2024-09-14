use serde::Serialize;
use std::fs::File;
use std::io::Write;

use crate::{
	default::*, utils::IDENT, Globals
};

#[derive(Serialize)]
struct Config {
	general : GeneralConfig,
	mutation: MutationConfig,
}

// Section générale du fichier TOML
#[derive(Serialize)]
struct GeneralConfig {
	version    : String,
	debug_level: u8,
}

// Section de configuration de la base de données
#[derive(Serialize)]
struct MutationConfig {
	test_cmd        : String,
	validator_node  : String,
	test_ledger_path: String,
	mutation_level  : u8,
}


pub fn toml_generation(
	g             : &Globals,
	test_cmd      : &str,
	validator_node: &str,
	mutation_level: u8
) -> std::io::Result<()> {
	let version = env!("CARGO_PKG_VERSION");
	let config = Config {
		general: GeneralConfig {
			version    : String::from( version),
			debug_level: DEFAULT_DEBUG_LEVEL,
		},
		mutation: MutationConfig {
			test_cmd        : String::from(DEFAULT_TEST_CMD),
			validator_node  : String::from(DEFAULT_VALIDATOR_NODE),
			test_ledger_path: String::from(DEFAULT_TEST_LEDGER_PATH),
			mutation_level  : DEFAULT_MUTATION_LEVEL,
		},
	};

	// Sérialiser la structure en une chaîne de caractères TOML
	let toml_str = toml::to_string(&config).expect("TOML Serialisation error");

	// Ouvrir ou créer le fichier de configuration TOML
	let toml_file: String = format!("{}/.mutatis/{}", g.fwd, "mutatis.toml");
	let mut fichier = File::create(toml_file)?;

	// Écrire la chaîne TOML dans le fichier
	fichier.write_all(toml_str.as_bytes())?;

	println!("{}TOML file generated", IDENT);
	Ok(())
}