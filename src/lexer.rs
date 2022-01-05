use crate::errors::{illegal_token, missing_closing_brace};
use crate::lexer::tokens::{Token, Scope, Symbol, GroupElem};
use crate::input::SourceFile;

pub mod tokens;

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
			illegal_token(Token::Symbol(self.next_symbol()), 
				Some("unexpected rougue closing-brace"), None);
		}
		if first == '\n' {
			self.index += 1;
			return Token::new_line_break(self.new_scope(start, end+1));
		}
		if is_sep_char(first) {
			self.index += 1;
			return Token::new_symbol(first, self.new_scope(start, end+1));
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
		Token::new_word(out, self.new_scope(start, end))
	}
	
	pub fn next_symbol(&mut self) -> Symbol {
		let sym = Symbol {
			value: self.chars[self.index],
			scope: self.new_scope(self.index, self.index+1),
		};
		self.index += 1;
		sym
	}
	
	pub fn next_group(&mut self, sep:char, closing:char) -> Token {
		let opening = self.next_symbol();
		
		let mut elems:Vec<GroupElem> = Vec::new();
		let mut tokens:Vec<Token> = Vec::new();
		
		while self.index+1 < self.size {
			let c = self.chars[self.index];
			if c == closing {
				let closing = self.next_symbol();
				
				if !tokens.is_empty() {
					elems.push(GroupElem {
						tokens:tokens.clone(),
						sep:None,
					});
				}
				
				return Token::new_group(
					opening,
					closing,
					elems,
				);
			}
			if c == sep {
				let sep = self.next_symbol();
				
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
		missing_closing_brace(opening.scope, self.index);
	}
	pub fn next_block(&mut self) -> Token {
		let opening = self.next_symbol();
		
		let mut tokens:Vec<Token> = Vec::new();
		
		while self.index+1 < self.size {
			let c = self.chars[self.index];
			if c == '}' {
				let closing = self.next_symbol();
				return Token::new_block(
					opening,
					closing,
					tokens
				);
			}
			
			if c != ' ' {
				let word = self.next();
				tokens.push(word);
			}
			else {
				self.skip_whitespace();
			}
		}
		missing_closing_brace(opening.scope, self.index);
	}

	pub fn skip_whitespace(&mut self) {
		let mut c = self.chars[self.index];
		while c == ' ' && self.index+1 < self.size {
			self.index += 1;
			c = self.chars[self.index];
		}
	}

	pub fn new_scope(&self, start:usize, end:usize) -> Scope {
		Scope {
			source_file: self.source_file.clone(),
			start, end
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
