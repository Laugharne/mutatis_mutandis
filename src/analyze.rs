use std::path::{Path, PathBuf};
use std::{fs, process};
use std::io;

use syn::ItemMod;
use syn::{parse_file, spanned::Spanned, BinOp, Expr, ExprUnary, File, UnOp};
use proc_macro2::Span;

use crate::{
	Globals,
	utils,
};


pub type IndexEntryPoint = u16;
pub type IndexMutation   = u16;
pub type MutationLevel   = u8;


#[derive(Debug)]
pub struct SourceCode {
	pub path_full    : PathBuf,
	pub path_src_root: String,
	pub file_name    : String,
	pub entry_point  : IndexEntryPoint, // number of entry point for code mutations
	pub done         : bool,
}


pub fn parse_directories(dir: &Path) -> io::Result<Vec<SourceCode>> {
	let org_dir: String = dir.display().to_string();

	match parse_directories_rec(&org_dir, dir, 0) {
		Ok(files) => Ok(files),
		Err(e) => {
			eprintln!("Error: {}", e);
			Err(e)
		}
	}
}

pub fn parse_directories_rec(org_dir: &str, dir: &Path, index: IndexMutation) -> io::Result<Vec<SourceCode>> {
	let mut files: Vec<SourceCode> = Vec::new();
	let skip               = org_dir.len() + 1;

	// Read directory content
	for entry in fs::read_dir(dir)? {
		let entry: fs::DirEntry = entry?;
		let path: PathBuf       = entry.path();

		if path.is_dir() {
			files.extend(parse_directories_rec(org_dir, &path, index)?);
		} else {
			let path_str: String = path.display().to_string();
			let file_ext: &str   = path_str.split('.').last().unwrap_or("");
			if file_ext.eq("rs") {
				let path_src_root: String = path_str.chars().skip(skip).collect();
				let file_name: &str       = path_str.split('/').last().unwrap_or("");

				files.push( SourceCode {
					path_full    : path,
					path_src_root: path_src_root,
					file_name    : file_name.to_owned(),
					entry_point  : 0,
					done         : false,
				});
			}
		}
	}

	Ok(files)
}


