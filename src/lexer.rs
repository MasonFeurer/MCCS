use crate::{Block, SourceRef, Sources, Token};

pub struct Lexer<'a> {
	pub main_source:usize,
	pub sources:&'a Sources,
	pub chars:Vec<char>,
	pub size:usize,
	pub index:usize,
	pub tokens:Vec<Token>,
}

impl<'a> Lexer<'a> {
	pub fn from(main:usize, sources:&'a Sources) -> Lexer<'a> {
		let source = sources.files[main].1.clone();
		let mut chars:Vec<char> = source.chars().collect();
		chars.push('\n');
		let size = chars.len();
		Lexer {
			main_source:main,
			sources,
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
			
		}
		if first == '\n' {
			self.index += 1;
			return Token::NewLine(self.src_ref(start-1, end));
		}
		if first.is_sep() {
			self.index += 1;
			return Token::Symbol(first, self.src_ref(start, end+1));
		} 

		while self.index < self.size {
			let c = self.chars[self.index];

			if c == '\n' || c.is_sep() || c == ' ' {
				break;
			}
			out.push(c);
			self.index += 1;
			end += 1;
		}
		Token::Word(out, self.src_ref(start, end))
	}
	
	fn last_char_word(&self) -> Token {
		Token::Word(self.chars[self.index].to_string(), 
		            self.src_ref(self.index, self.index+1))
	}
	
	pub fn next_group(&mut self, sep:char) -> Token {
		
	}
	pub fn next_block(&mut self) -> Token {
		let opening = self.last_char_word();
		self.index += 1;
		
		let mut tokens:Vec<Token> = Vec::new();
		
		while self.index+1 < self.size {
			let c = self.chars[self.index];
			if c == '}' {
				let closing = self.last_char_word();
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
		
		todo!()
	}

	pub fn skip_whitespace(&mut self) {
		let mut c = self.chars[self.index];
		while c == ' ' && self.index+1 < self.size {
			self.index += 1;
			c = self.chars[self.index];
		}
	}

	pub fn src_ref(&self, start:usize, end:usize) -> SourceRef {
		SourceRef { file:self.sources.files[self.main_source].0.clone(), start, end }
	}
}

trait CharTools {
	fn is_sep(&self) -> bool;
}
impl CharTools for char {
	fn is_sep(&self) -> bool {
		let c = *self;
		c == '(' || c == ')' ||
			c == '{' || c == '}' ||
			c == '[' || c == ']' ||
			c == ';' || c == ',' ||
			c == '.' || c == '!' ||
			c == '|' || c == ':'
	}
}
