use serde::{
	Serialize,
	Deserialize
};
use std::fs;
use std::{fs::File, path::Path};
use std::io::Write;

use crate::{
	default::*, utils::IDENT, Globals,
	analyze::MutationLevel,
};

#[derive(Serialize, Debug, Deserialize)]
pub struct MainConfig {
	pub general : MainGeneralConfig,
	pub mutation: MainMutationConfig,
}

// Section générale du fichier TOML
#[derive(Serialize, Debug, Deserialize)]
pub struct MainGeneralConfig {
	pub version    : String,
	pub debug_level: u8,
}

// Section de configuration de la base de données
#[derive(Serialize, Debug, Deserialize)]
pub struct MainMutationConfig {
	pub test_cmd        : String,
	pub validator_node  : String,
	pub test_ledger_path: String,
	pub mutation_level  : MutationLevel,
	pub validator_pause : u8,
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
			test_cmd        : String::from(test_cmd),
			validator_node  : String::from(validator_node),
			validator_pause : DEFAULT_VALIDATOR_PAUSE,
			test_ledger_path: String::from(DEFAULT_TEST_LEDGER_PATH),
			mutation_level  : mutation_level,
			//test_cmd        : String::from(DEFAULT_TEST_CMD),
			//validator_node  : String::from(DEFAULT_VALIDATOR_NODE),
			//test_ledger_path: String::from(DEFAULT_TEST_LEDGER_PATH),
			//mutation_level  : DEFAULT_MUTATION_LEVEL,
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


pub fn main_toml_read(
	g: &Globals,
) -> std::io::Result<(MainConfig)> {
	let toml_file: String  = format!("{}/.mutatis/{}", g.fwd, "mutatis.toml");
	let content: String    = fs::read_to_string(toml_file)?;
	let config: MainConfig = toml::from_str(&content).unwrap();
	//println!("{:#?}", config);
	Ok(config)
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MutationConfig {
	general : MutationGeneralConfig,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct MutationGeneralConfig {
	full_file_path: String,
	//id: String,	// TODO
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

pub fn mutation_toml_read(
	g           : &Globals,
	mutation_toml: &str
) -> std::io::Result<(MutationConfig)> {
	let content: String        = fs::read_to_string(mutation_toml)?;
	let config: MutationConfig = toml::from_str(&content).unwrap();
	//println!("{:#?}", config);
	Ok(config)
}