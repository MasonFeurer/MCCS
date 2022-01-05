use std::fmt::{Display, Formatter};
use crate::lexer::tokens::{Token, Word, Block};
use crate::errors::{illegal_token, Help};

pub struct Parser {
	pub tokens:Vec<Token>,
	pub index:usize,
	pub size:usize,
	pub datapack_name_decl:Option<Word>,
	pub parsed_functions:Vec<ParsedFunction>,
}
impl Parser {
	pub fn new(tokens:Vec<Token>) -> Parser {
		let size = tokens.len();
		Parser {
			tokens,
			index: 0,
			size,
			datapack_name_decl:None,
			parsed_functions:Vec::new(),
		}
	}
	
	fn next_ident(&mut self) -> Word {
		let token = self.tokens[self.index].clone();
		self.index += 1;
		match token {
			Token::Word(word) => { word }
			_ => { illegal_token(token, Some("expected identifier"), None) }
		}
	}
	fn next_token(&mut self) -> Token {
		let token = self.tokens[self.index].clone();
		self.index += 1;
		token
	}
	
	fn expect_line_break(&mut self) {
		let next = self.tokens[self.index].clone();
		if let Token::LineBreak(_) = next { return }
		illegal_token(next, Some("expected end of line"), None);
	}
	
	pub fn parse(&mut self) {
		let mut mods:Vec<Word> = Vec::new();
		
		while self.index < self.size {
			let token = self.next_token();
			
			match token {
				Token::LineBreak(_) => { continue }
				Token::Word(ref word) => {
					let value = &word.value[..];
					
					if value == "inline" || value == "onload"
					|| value == "ontick" || value == "cmpdmp"
					|| value == "hidden" {
						mods.push(word.clone());
						continue;
					}
					
					if value == "datapack" {
						self.datapack_name_decl = Some(self.next_ident());
						self.expect_line_break();
						continue;
					}
					if value == "fn" {
						if self.datapack_name_decl.is_none() {
							illegal_token(token, 
								Some("must declare datapack first"), 
								Some(Help {
									message: "try declaring datapack".to_string(),
									source: "datapack my_datapack".to_string(),
									line: 1,
								}));
						}
						let name = self.next_ident();
						
						let next = self.next_token();
						let block = if let Token::Block(block) = next {
							*block
						}
						else { illegal_token(next, Some("expected block"), None) };
						
						self.parsed_functions.push(ParsedFunction {
							name,
							mods:mods.clone(),
							block,
						});
						mods.clear();
						continue;
					}
					illegal_token(token, Some("expected item declaration"), None);
					
				}
				_ => {
					illegal_token(token, Some("expected item declaration"), None);
				}
			}
		}
	}
}

#[derive(Debug)]
pub struct ParsedFunction {
	pub name:Word,
	pub mods:Vec<Word>,
	//pub input:Group,
	//pub output:Group,
	pub block:Block,
}
impl Display for ParsedFunction {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(format!(
			"{} function {}",
			elems_to_string(&self.mods, |e| { e.value.clone() }),
			self.name.value,
		).as_str())
	}
}

pub fn elems_to_string<T, F>(vec:&Vec<T>, to_string:F) -> String 
where F: Fn(&T) -> String {
	let mut string = String::new();
	string.push('[');
	
	if !vec.is_empty() {
		string.push_str(to_string(&vec[0]).as_str());
	}
	for e in vec.into_iter().skip(1) {
		string.push_str(", ");
		string.push_str(to_string(e).as_str());
	}
	
	string.push(']');
	string
}
