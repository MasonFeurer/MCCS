#![allow(dead_code)]

pub mod errors;
pub mod lexer;
pub mod sources;
pub mod parser;

use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use crate::lexer::Lexer;
use crate::sources::{SourceRef, Sources};

pub fn main() {
	let mut sources = Sources::new();
	
	let mut file1 = File::open("/home/mason/Desktop/some_code.mccs").expect("failed to open file");
	let mut source1 = String::new();
	file1.read_to_string(&mut source1).expect("failed to read file");
	source1 = source1.replace("\t", "    ");
	source1.push('\n');
	
	sources.push_source("some_code.mccs".to_string(), source1);
	
	let mut lexer = Lexer::from(0, &sources);
	lexer.lex();
	
	for t in lexer.tokens {
		//let debug = debug_token(t, &sources, None);
		println!("{}", t);
	}
}

pub enum Token {
	Word(String, SourceRef),
	Symbol(char, SourceRef),
	NewLine(SourceRef),
	Block(Box<Block>),
	Group(Box<Group>),
}
impl Token {
	pub fn first_src_ref(&self) -> &SourceRef {
		match self {
			Token::Word(_, src_ref) => { src_ref }
			Token::Symbol(_, src_ref) => { src_ref }
			Token::NewLine(src_ref) => { src_ref }
			Token::Block(block) => {
				block.opening.first_src_ref()
			}
			Token::Group(group) => {
				group.opening.first_src_ref()
			}
		}
	}
}
impl Display for Token {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Word(word, _) => {
				f.write_str(format!("Word '{}'", word).as_str())
			}
			Token::Symbol(ch, _) => {
				f.write_str(format!("Symbol '{}'", ch).as_str())
			}
			Token::NewLine(_) => {
				f.write_str("Line Break")
			}
			Token::Block(block) => {
				let block = &**block;
				let mut string = String::new();
				string.push_str("Block");
				for t in &block.tokens {
					string.push_str("\n\t");
					string.push_str(t.to_string().replace("\n", "\n\t").as_str());
				}
				f.write_str(string.as_str())
			}
			Token::Group(group) => {
				let group = &**group;
				let mut string = String::new();
				string.push_str("Group");
				for e in &group.elems {
					string.push_str("GroupElem:");
					for t in &e.tokens {
						string.push_str("\n\t");
						string.push_str(t.to_string().replace("\n", "\n\t").as_str());
					}
				}
				f.write_str(string.as_str())
			}
		}
	}
}

pub struct Block {
	opening:Token,
	closing:Token,
	tokens:Vec<Token>,
}
pub struct Group {
	opening:Token,
	closing:Token,
	elems:Vec<GroupElem>,
}
pub struct GroupElem {
	tokens:Vec<Token>,
	sep:Option<Token>,
}
