use std::path::{Path, PathBuf};
use std::{fs, process};
use std::io;

use syn::{
	ItemMod,
	parse_file,
	spanned::Spanned,
	BinOp,
	Expr,
	ExprUnary,
	File,
	UnOp
};
use proc_macro2::Span;

use crate::toml::mutation_toml_generation;
use crate::{
	Globals,
	utils::{
		build_mutation_index_str,
		shell_call
	},
	analyze::{
		IndexEntryPoint,
		IndexMutation,
		SourceCode,
	},

};

pub fn pass2(
	g      : &Globals,
	src_dir: &str,
	files  : &mut Vec<SourceCode>
) {

	//println!("{}", src_dir);// no '/' at the end

	let mut mutation_index: IndexMutation = 0;

	for file in files.iter_mut() {
		//println!("{}{}{} {}", IDENT, IDENT, "-".red(), file.path_src_root);
		let input_path: &Path = file.path_full.as_ref();
		let code: String      = fs::read_to_string(input_path).expect("Unable to read file");

		// Parse code into AST
		let ast: File                              = syn::parse_file(&code).expect("Unable to parse file");
		let mut entry_point_index: IndexEntryPoint = 0;
		let entry_point_mutation: IndexEntryPoint  = file.entry_point;

		(0..entry_point_mutation).for_each( |entry| {
			let dir_mutation: String          = build_mutation_index_str(entry);
			let full_mutation_sub_dir: String = format!("{}/.mutatis/mutations/{}", g.fwd, dir_mutation);
			//println!("{}",  full_mutation_sub_dir);
			shell_call("mkdir", &full_mutation_sub_dir);
			let full_mutation_toml_file: String = format!(
				"{}/.mutatis/mutations/{}/{}_{}.toml",
				g.fwd,
				dir_mutation,
				dir_mutation,
				file.file_name
			);
			//println!("{}",  full_mutation_toml_file);
			let _ = mutation_toml_generation(
				&g,
				&full_mutation_toml_file,
				input_path.to_str().unwrap()
			);

			let ouput_path: String = format!("{}/{}", full_mutation_sub_dir, file.file_name);
			let output_path: &Path = Path::new(&ouput_path);
			let ast_output: File   = pass2_parse_ast(ast.clone(), entry_point_mutation, &mut entry_point_index);

			let modified_code: String = prettyplease::unparse(&ast_output);
			fs::write(output_path, modified_code).expect("Unable to write file");
	});


	}

}

fn pass2_parse_ast(
	mut ast                 : File,
	    entry_point_mutation: IndexEntryPoint,
	    entry_point_index   : &mut IndexEntryPoint
) -> File {
	// Parcourir les items de l'AST
	for item in &mut ast.items {
		if let syn::Item::Fn(func) = item {
			// Modifier les expressions dans le corps de la fonction
			for statement in &mut func.block.stmts {
				pass2_parse_stmt(statement, entry_point_mutation, entry_point_index);
			}
			pass2_parse_function(func, entry_point_mutation, entry_point_index);
		}
		match item {
			syn::Item::Fn(func) => {
				for statement in &mut func.block.stmts {
					pass2_parse_stmt(statement, entry_point_mutation, entry_point_index);
				}
				pass2_parse_function(func, entry_point_mutation, entry_point_index);
			},
			syn::Item::Mod(module) => {
				pass2_parse_module(module, entry_point_mutation, entry_point_index);
			}
			_ => {}
		}
	}
	ast
}



fn pass2_parse_module(
	module              : &mut ItemMod,
	entry_point_mutation: IndexEntryPoint,
	entry_point_index   : &mut IndexEntryPoint
) {
	// Check if the module has content to process (functions inside)
	if let Some((_, items)) = &mut module.content {
		for item in items {
			if let syn::Item::Fn(func) = item {
				pass2_parse_function(func, entry_point_mutation, entry_point_index);
			}
		}
	}
}


fn pass2_parse_function(
	func                : &mut syn::ItemFn,
	entry_point_mutation: IndexEntryPoint,
	entry_point_index   : &mut IndexEntryPoint
) {
	// Vérifie et modifie la dernière expression du bloc (retour implicite)
	if let Some(last_expr) = func.block.stmts.last_mut() {
		if let syn::Stmt::Expr(expr) = last_expr {
			pass2_parse_boolean_literal(expr, entry_point_mutation, entry_point_index);
		}
	}

	// Modify statements in the function body
	for statement in &mut func.block.stmts {
		pass2_parse_stmt(statement, entry_point_mutation, entry_point_index);
	}

}



fn pass2_parse_stmt(
	statement           : &mut syn::Stmt,
	entry_point_mutation: IndexEntryPoint,
	entry_point_index   : &mut IndexEntryPoint
) {
	if let syn::Stmt::Expr(expr) | syn::Stmt::Semi(expr, _) = statement {
		pass2_parse_expr(expr, entry_point_mutation, entry_point_index);
	}
}

fn pass2_parse_expr(
	expr                : &mut syn::Expr,
	entry_point_mutation: IndexEntryPoint,
	entry_point_index   : &mut IndexEntryPoint
) {
	match expr {
		// Si l'expression est une condition if, elle est modifiée via `pass2_parse_condition()`.
		syn::Expr::If(expr_if) => {
			pass2_parse_condition(&mut expr_if.cond, entry_point_mutation, entry_point_index);

			// Les branches `then` et else `des` instructions `if`
			// sont également parcourues pour appliquer les modifications.
			for statement in &mut expr_if.then_branch.stmts {
				pass2_parse_stmt(statement, entry_point_mutation, entry_point_index);
			}
			if let Some((_, else_branch)) = &mut expr_if.else_branch {
				pass2_parse_expr(else_branch, entry_point_mutation, entry_point_index);
			}
		}
		// Recursively modify other expressions
		syn::Expr::Block(expr_block) => {
			for statement in &mut expr_block.block.stmts {
				pass2_parse_stmt(statement, entry_point_mutation, entry_point_index);
			}
		}

		// Si l'expression est un retour booléen, inverser la valeur retournée
		syn::Expr::Return(ret) => {
			if let Some(expr) = &mut ret.expr {
				pass2_parse_boolean_literal(expr, entry_point_mutation, entry_point_index);
			}
		}

		_ => {}
	}
}



fn pass2_parse_condition(
	cond                : &mut syn::Expr,
	entry_point_mutation: IndexEntryPoint,
	entry_point_index   : &mut IndexEntryPoint
) {
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
				*entry_point_index += 1;
			}

			// Si l'opérateur est `>`, il est remplacé par `<=` (inférieur ou égal).
			BinOp::Gt(_) => {
				// Récupère la position (span) de l'opérateur pour
				// préserver cette information lors de la modification.

				let span: Span = expr_binary.op.span();
				expr_binary.op = BinOp::Le(syn::token::Le {
					spans: [span, span],
				});
				*entry_point_index += 1;
			}

			_ => {}
		}
		// Récursivement, modifier les sous-expressions gauche et droite si nécessaire
		pass2_parse_condition(&mut *expr_binary.left,  entry_point_mutation, entry_point_index);
		pass2_parse_condition(&mut *expr_binary.right, entry_point_mutation, entry_point_index);

	}

	// Si la condition est une expression unaire (comme `!a`), appliquer la modification récursive.
	if let syn::Expr::Unary(expr_unary) = cond {
		pass2_parse_condition(&mut *expr_unary.expr, entry_point_mutation, entry_point_index);
	}

}

fn pass2_parse_boolean_literal(
	expr                : &mut syn::Expr,
	entry_point_mutation: IndexEntryPoint,
	entry_point_index   : &mut IndexEntryPoint
) {
	if let syn::Expr::Lit(expr_lit) = expr {
		if let syn::Lit::Bool(ref mut lit_bool) = expr_lit.lit {
			lit_bool.value = !lit_bool.value;  // Inverse la valeur booléenne
			*entry_point_index += 1;
		}
	}
}

