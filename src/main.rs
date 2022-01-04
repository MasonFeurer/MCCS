#![allow(dead_code)]

pub mod errors;
pub mod lexer;
pub mod parser;
pub mod bash_tools;

use std::fs::File;
use std::io::Read;
use crate::lexer::{Lexer, SourceFile};

pub fn main() {
	let mut file = File::open("/Users/745832/Desktop/some_code.mccs").expect("failed to open file");
	let mut source = String::new();
	file.read_to_string(&mut source).expect("failed to read file");
	source = source.replace("\t", "    ");
	source.push('\n');
	
	// must make sure the `source` doesn't get dropped before `source_file`.
	// in this case, they both get dropped at the end of `main()`
	let source_file = SourceFile::new("src/some_code.mccs".to_string(), &source);
	
	let mut lexer = Lexer::new(source_file);
	lexer.lex();
	
	for t in lexer.tokens {
		println!("{}", t);
	}
}
