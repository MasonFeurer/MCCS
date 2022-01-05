use std::fmt::{Display, Formatter};
use crate::input::SourceFile;

#[derive(Clone, Debug)]
pub enum Token {
	Word(Word),
	Symbol(Symbol),
	LineBreak(LineBreak),
	Block(Box<Block>),
	Group(Box<Group>),
}
impl Token {
	pub fn get_scope(&self) -> Scope {
		match self {
			Token::Word(word) => { word.scope.clone() }
			Token::Symbol(symbol) => { symbol.scope.clone() }
			Token::LineBreak(line_break) => { line_break.scope.clone() }
			Token::Block(block) => { block.opening.scope.clone() }
			Token::Group(group) => { group.opening.scope.clone() }
		}
	}
	
	pub fn new_word(value:String, scope:Scope) -> Token {
		Token::Word(Word { value, scope })
	}
	pub fn new_symbol(value:char, scope:Scope) -> Token {
		Token::Symbol(Symbol { value, scope })
	}
	pub fn new_line_break(scope:Scope) -> Token {
		Token::LineBreak(LineBreak { scope })
	}
	pub fn new_block(opening:Symbol, closing:Symbol, tokens:Vec<Token>) -> Token {
		Token::Block(Box::new(Block { opening, closing, tokens }))
	}
	pub fn new_group(opening:Symbol, closing:Symbol, elems:Vec<GroupElem>) -> Token {
		Token::Group(Box::new(Group { opening, closing, elems }))
	}
}
impl Display for Token {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Word(word) => {
				f.write_str(format!("Word '{}'", word.value).as_str())
			}
			Token::Symbol(symbol) => {
				f.write_str(format!("Symbol '{}'", symbol.value).as_str())
			}
			Token::LineBreak(_) => {
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
					string.push_str("\n:Elem:");
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

#[derive(Clone, Debug)]
pub struct Word {
	pub value:String,
	pub scope:Scope,
}
#[derive(Clone, Debug)]
pub struct Symbol {
	pub value:char,
	pub scope:Scope,
}
#[derive(Clone, Debug)]
pub struct LineBreak {
	pub scope:Scope,
}
#[derive(Clone, Debug)]
pub struct Block {
	pub opening:Symbol,
	pub closing:Symbol,
	pub tokens:Vec<Token>,
}
#[derive(Clone, Debug)]
pub struct Group {
	pub opening:Symbol,
	pub closing:Symbol,
	pub elems:Vec<GroupElem>,
}

#[derive(Clone, Debug)]
pub struct GroupElem {
	pub tokens:Vec<Token>,
	pub sep:Option<Symbol>,
}

#[derive(Clone, Debug)]
pub struct Scope {
	pub source_file:SourceFile,
	pub start:usize,
	pub end:usize,
}
