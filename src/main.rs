#![allow(dead_code)]

pub mod errors;
pub mod lexer;
pub mod parser;
pub mod bash_tools;
pub mod input;

use std::fs::File;
use std::io::Read;
use std::env;
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::input::SourceFile;

pub fn main() {
	let args:Vec<String> = env::args().collect();
	
	if args.len() != 2 {
		panic!("Must supply one source file");
	}
	
	let file_path_arg = args[1].clone();
	
	let mut file = File::open(file_path_arg.as_str()).expect("failed to open file");
	let mut source = String::new();
	file.read_to_string(&mut source).expect("failed to read file");
	source = source.replace("\t", "    ");
	source.push('\n');
	
	// must make sure the `source` doesn't get dropped before `source_file`.
	// in this case, they both get dropped at the end of `main()`
	let source_file = SourceFile::new(file_path_arg, &source);
	
	let mut lexer = Lexer::new(source_file);
	lexer.lex();
	
	let mut parser = Parser::new(lexer.tokens);
	parser.parse();
	
	for f in parser.parsed_functions {
		println!("{}", f);
	}
	
	println!("DONE");
}
