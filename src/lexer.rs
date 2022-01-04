use crate::errors::{illegal_token, missing_closing_brace};
use std::fmt::{Display, Formatter};

pub struct Lexer {
	pub source_file:SourceFile,
	pub chars:Vec<char>,
	pub size:usize,
	pub index:usize,
	pub tokens:Vec<Token>,
}

impl Lexer {
	pub fn new(source_file:SourceFile) -> Lexer {
		let mut chars:Vec<char> = source_file.get_source().chars().collect();
		chars.push('\n');
		let size = chars.len();
		Lexer {
			source_file,
			chars,
			size,
			index:0,
			tokens:Vec::new()
		}
	}

	pub fn lex(&mut self) {
		while self.index+1 < self.size {
			let c = self.chars[self.index];
			if c != ' ' {
				let word = self.next();
				self.tokens.push(word);
			}
			else {
				self.skip_whitespace();
			}
		}
	}

	pub fn next(&mut self) -> Token {
		let mut out = String::new();
		let start = self.index;
		let mut end = start;
		
		let first = self.chars[self.index];

		if first == '{' {
			return self.next_block();
		}
		if first == '(' {
			return self.next_group(',', ')');
		}
		if first == '}' || first == ')' {
			illegal_token(self.last_as_token(), 
				Some("unexpected rougue closing-brace"));
		}
		if first == '\n' {
			self.index += 1;
			return Token::NewLine(self.new_scope(start-1, end));
		}
		if is_sep_char(first) {
			self.index += 1;
			return Token::Symbol(first, self.new_scope(start, end+1));
		} 

		while self.index < self.size {
			let c = self.chars[self.index];

			if c == '\n' || is_sep_char(c) || c == ' ' {
				break;
			}
			out.push(c);
			self.index += 1;
			end += 1;
		}
		Token::Word(out, self.new_scope(start, end))
	}
	
	pub fn next_group(&mut self, sep:char, closing:char) -> Token {
		let opening = self.last_as_token();
		self.index += 1;
		
		let mut elems:Vec<GroupElem> = Vec::new();
		let mut tokens:Vec<Token> = Vec::new();
		
		while self.index+1 < self.size {
			let c = self.chars[self.index];
			if c == closing {
				let closing = self.last_as_token();
				self.index += 1;
				
				if !tokens.is_empty() {
					elems.push(GroupElem {
						tokens:tokens.clone(),
						sep:None,
					});
				}
				
				return Token::Group(Box::new(Group {
					opening,
					closing,
					elems,
				}));
			}
			if c == sep {
				let sep = self.last_as_token();
				self.index += 1;
				
				elems.push(GroupElem {
					tokens:tokens.clone(),
					sep:Some(sep),
				});
				tokens.clear();
			}
			else if c != ' ' {
				let word = self.next();
				tokens.push(word);
			}
			else {
				self.skip_whitespace();
			}
		}
		missing_closing_brace(opening.get_scope(), self.index);
	}
	pub fn next_block(&mut self) -> Token {
		let opening = self.last_as_token();
		self.index += 1;
		
		let mut tokens:Vec<Token> = Vec::new();
		
		while self.index+1 < self.size {
			let c = self.chars[self.index];
			if c == '}' {
				let closing = self.last_as_token();
				self.index += 1;
				return Token::Block(Box::new(Block {
					opening,
					closing,
					tokens
				}));
			}
			
			if c != ' ' {
				let word = self.next();
				tokens.push(word);
			}
			else {
				self.skip_whitespace();
			}
		}
		missing_closing_brace(opening.get_scope(), self.index);
	}

	pub fn skip_whitespace(&mut self) {
		let mut c = self.chars[self.index];
		while c == ' ' && self.index+1 < self.size {
			self.index += 1;
			c = self.chars[self.index];
		}
	}

	pub  fn new_scope(&self, start:usize, end:usize) -> Scope {
		Scope {
			source_file: self.source_file.clone(),
			start, end
		}
	}
	pub fn last_as_token(&self) -> Token {
		let c = self.chars[self.index];
		let scope = self.new_scope(self.index, self.index+1);
		
		if is_sep_char(c) {
			Token::Symbol(c, scope)
		}
		else {
			Token::Word(c.to_string(), scope)
		}
	}
}

fn is_sep_char(c:char) -> bool {
	c == '(' || c == ')' ||
	c == '{' || c == '}' ||
	c == '[' || c == ']' ||
	c == ';' || c == ',' ||
	c == '.' || c == '!' ||
	c == '|' || c == ':'
}

#[derive(Clone)]
pub enum Token {
	Word(String, Scope),
	Symbol(char, Scope),
	NewLine(Scope),
	Block(Box<Block>),
	Group(Box<Group>),
}
impl Token {
	pub fn get_scope(&self) -> &Scope {
		match self {
			Token::Word(_, scope) => { scope }
			Token::Symbol(_, scope) => { scope }
			Token::NewLine(scope) => { scope }
			Token::Block(block) => {
				block.opening.get_scope()
			}
			Token::Group(group) => {
				group.opening.get_scope()
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

#[derive(Clone)]
pub struct SourceFile {
	pub path:String,
	// Using a raw pointer because the lifetime parameters 
	// cascades up the type system, and just looks messy
	pub p_source:*const String,
}
impl SourceFile {
	// IMPORTANT: The passed String ref must last atleast 
	// as long as the resulting struct
	pub fn new(path:String, source:&String) -> Self {
		SourceFile {
			path,
			p_source: source as *const String
		}
	}
	
	pub fn get_source(&self) -> &String {
		unsafe { &*self.p_source }
	}
}

#[derive(Clone)]
pub struct Scope {
	pub source_file:SourceFile,
	pub start:usize,
	pub end:usize,
}
impl Display for Scope {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(format!("{{file '{}', {}->{}}}", self.source_file.path, self.start, self.end).as_str())
	}
}

#[derive(Clone)]
pub struct Block {
	opening:Token,
	closing:Token,
	tokens:Vec<Token>,
}
#[derive(Clone)]
pub struct Group {
	opening:Token,
	closing:Token,
	elems:Vec<GroupElem>,
}
#[derive(Clone)]
pub struct GroupElem {
	tokens:Vec<Token>,
	sep:Option<Token>,
}
