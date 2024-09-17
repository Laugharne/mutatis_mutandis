use serde::Serialize;
use std::{fs::File, path::Path};
use std::io::Write;

use crate::{
	default::*, utils::IDENT, Globals,
	analyze::MutationLevel,
};

#[derive(Serialize)]
struct MainConfig {
	general : MainGeneralConfig,
	mutation: MainMutationConfig,
}

// Section générale du fichier TOML
#[derive(Serialize)]
struct MainGeneralConfig {
	version    : String,
	debug_level: u8,
}

// Section de configuration de la base de données
#[derive(Serialize)]
struct MainMutationConfig {
	test_cmd        : String,
	validator_node  : String,
	test_ledger_path: String,
	mutation_level  : MutationLevel,
}


pub fn main_toml_generation(
	g             : &Globals,
	test_cmd      : &str,
	validator_node: &str,
	mutation_level: MutationLevel
) -> std::io::Result<()> {
	let version = env!("CARGO_PKG_VERSION");
	let config = MainConfig {
		general: MainGeneralConfig {
			version    : String::from( version),
			debug_level: DEFAULT_DEBUG_LEVEL,
		},
		mutation: MainMutationConfig {
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



#[derive(Serialize)]
struct MutationConfig {
	general : MutationGeneralConfig,
}

#[derive(Serialize)]
struct MutationGeneralConfig {
	full_file_path: String,
}

pub fn mutation_toml_generation(
	g             : &Globals,
	full_path_toml: &str,
	full_path_src : &str,
) -> std::io::Result<()> {

	let config = MutationConfig {
		general: MutationGeneralConfig {
		    full_file_path: String::from( full_path_src),
		},
	};

	// Sérialiser la structure en une chaîne de caractères TOML
	let toml_str = toml::to_string(&config).expect("TOML Serialisation error");

	// Ouvrir ou créer le fichier de configuration TOML
	let mut fichier = File::create(full_path_toml)?;

	// Écrire la chaîne TOML dans le fichier
	fichier.write_all(toml_str.as_bytes())?;

	Ok(())
}