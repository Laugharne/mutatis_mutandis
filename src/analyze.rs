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


pub fn pass1(g: &Globals, src_dir: &str, files: &mut Vec<SourceCode>) {

	//println!("{}", src_dir);// no '/' at the end

	for file in files.iter_mut() {
		//println!("{}{}{} {}", IDENT, IDENT, "-".red(), file.path_src_root);
		let input_path: &Path = file.path_full.as_ref();
		let code: String = fs::read_to_string(input_path).expect("Unable to read file");

		// Parse code into AST
		let ast: File = syn::parse_file(&code).expect("Unable to parse file");
		let mut nn_entry_point: IndexEntryPoint = 0;

		let ast_output: File = pass1_parse_ast(ast, &mut nn_entry_point);

		// let modified_code: String = prettyplease::unparse(&ast_output);
		// let output_path: &Path = Path::new("./__modified.rs");
		// fs::write(output_path, modified_code).expect("Unable to write file");

		file.entry_point = nn_entry_point;
	}

}


fn pass1_parse_ast(mut ast: File, nn_entry_point: &mut IndexEntryPoint) -> File {
	// Parcourir les items de l'AST
	for item in &mut ast.items {
		if let syn::Item::Fn(func) = item {
			// Modifier les expressions dans le corps de la fonction
			for statement in &mut func.block.stmts {
				pass1_parse_stmt(statement, nn_entry_point);
			}
			pass1_parse_function(func, nn_entry_point);
		}
		match item {
			syn::Item::Fn(func) => {
				for statement in &mut func.block.stmts {
					pass1_parse_stmt(statement, nn_entry_point);
				}
				pass1_parse_function(func, nn_entry_point);
			},
			syn::Item::Mod(module) => {
				pass1_parse_module(module, nn_entry_point);
			}
			_ => {}
		}
	}
	ast
}

fn pass1_parse_module(module: &mut ItemMod, nn_entry_point: &mut IndexEntryPoint) {
	// Check if the module has content to process (functions inside)
	if let Some((_, items)) = &mut module.content {
		for item in items {
			if let syn::Item::Fn(func) = item {
				pass1_parse_function(func, nn_entry_point);
			}
		}
	}
}


fn pass1_parse_function(func: &mut syn::ItemFn, nn_entry_point: &mut IndexEntryPoint) {
	// Vérifie et modifie la dernière expression du bloc (retour implicite)
	if let Some(last_expr) = func.block.stmts.last_mut() {
		if let syn::Stmt::Expr(expr) = last_expr {
			pass1_parse_boolean_literal(expr, nn_entry_point);
		}
	}

	// Modify statements in the function body
	for statement in &mut func.block.stmts {
		pass1_parse_stmt(statement, nn_entry_point);
	}

}

fn pass1_parse_stmt(statement: &mut syn::Stmt, nn_entry_point: &mut IndexEntryPoint) {
	if let syn::Stmt::Expr(expr) | syn::Stmt::Semi(expr, _) = statement {
		pass1_parse_expr(expr, nn_entry_point);
	}
}

fn pass1_parse_expr(expr: &mut syn::Expr, nn_entry_point: &mut IndexEntryPoint) {
	match expr {
		// Si l'expression est une condition if, elle est modifiée via `pass1_parse_condition()`.
		syn::Expr::If(expr_if) => {
			pass1_parse_condition(&mut expr_if.cond, nn_entry_point);

			// Les branches `then` et else `des` instructions `if`
			// sont également parcourues pour appliquer les modifications.
			for statement in &mut expr_if.then_branch.stmts {
				pass1_parse_stmt(statement, nn_entry_point);
			}
			if let Some((_, else_branch)) = &mut expr_if.else_branch {
				pass1_parse_expr(else_branch, nn_entry_point);
			}
		}
		// Recursively modify other expressions
		syn::Expr::Block(expr_block) => {
			for statement in &mut expr_block.block.stmts {
				pass1_parse_stmt(statement, nn_entry_point);
			}
		}

		// Si l'expression est un retour booléen, inverser la valeur retournée
		syn::Expr::Return(ret) => {
			if let Some(expr) = &mut ret.expr {
				pass1_parse_boolean_literal(expr, nn_entry_point);
			}
		}

		_ => {}
	}
}

fn pass1_parse_condition(cond: &mut syn::Expr, nn_entry_point: &mut IndexEntryPoint) {
	// Si la condition est une expression binaire (comme `a == b`), elle est modifiée.
	if let syn::Expr::Binary(expr_binary) = cond {
		match expr_binary.op {

			// Si l'opérateur est `==`, il est remplacé par `!=` (différent).
			BinOp::Eq(_) => {
				// Récupère la position (span) de l'opérateur pour
				// préserver cette information lors de la modification.

				let span: Span = expr_binary.op.span();
				expr_binary.op = BinOp::Ne(syn::token::Ne {
					spans: [span, span],
				});
				*nn_entry_point += 1;
			}

			// Si l'opérateur est `>`, il est remplacé par `<=` (inférieur ou égal).
			BinOp::Gt(_) => {
				// Récupère la position (span) de l'opérateur pour
				// préserver cette information lors de la modification.

				let span: Span = expr_binary.op.span();
				expr_binary.op = BinOp::Le(syn::token::Le {
					spans: [span, span],
				});
				*nn_entry_point += 1;
			}

			_ => {}
		}
		// Récursivement, modifier les sous-expressions gauche et droite si nécessaire
		pass1_parse_condition(&mut *expr_binary.left, nn_entry_point);
		pass1_parse_condition(&mut *expr_binary.right, nn_entry_point);

	}

	// Si la condition est une expression unaire (comme `!a`), appliquer la modification récursive.
	if let syn::Expr::Unary(expr_unary) = cond {
		pass1_parse_condition(&mut *expr_unary.expr, nn_entry_point);
	}

}

fn pass1_parse_boolean_literal(expr: &mut syn::Expr, nn_entry_point: &mut IndexEntryPoint) {
	if let syn::Expr::Lit(expr_lit) = expr {
		if let syn::Lit::Bool(ref mut lit_bool) = expr_lit.lit {
			lit_bool.value = !lit_bool.value;  // Inverse la valeur booléenne
			*nn_entry_point += 1;
		}
	}
}


