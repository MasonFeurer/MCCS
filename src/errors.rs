use std::cmp::max;
use std::process::exit;
use crate::{Block, SourceRef, Sources, Token};


pub fn illegal_token(token:Token, sources:&Sources, more:Option<&str>) -> ! {
	eprintln!("ILLEGAL TOKEN");
	eprintln!("{}", debug_token(token, sources, more));
	exit(1);
}
pub fn debug_token(token:Token, sources:&Sources, more:Option<&str>) -> String {
	match token {
		Token::Word(word, src_ref) => {
			debug_word(word, src_ref, sources, more)
		}
		Token::Symbol(symbol, src_ref) => {
			debug_symbol(symbol, src_ref, sources, more)
		}
		Token::NewLine(src_ref) => {
			debug_new_line(src_ref, sources, more)
		}
		Token::Block(block) => {
			debug_block(*block, sources)
		}
	}
}
pub fn debug_word(word:String, src_ref:SourceRef, sources:&Sources, more:Option<&str>) -> String {
	let source = sources.source_by_name(src_ref.file.clone()).expect("source not found");
	let line_num = get_line_num(src_ref.start, &source);
	let underlined = underline_word(&src_ref, line_num, source, more);
	format!("word '{}' \n{}\n", word, underlined)
}
pub fn debug_symbol(symbol:char, src_ref:SourceRef, sources:&Sources, more:Option<&str>) -> String {
	let source = sources.source_by_name(src_ref.file.clone()).expect("source not found");
	let line_num = get_line_num(src_ref.start, &source);
	let underlined = underline_word(&src_ref, line_num, source, more);
	format!("symbol '{}' \n{}\n", symbol, underlined)
}
fn debug_new_line(src_ref:SourceRef, sources:&Sources, more:Option<&str>) -> String {
	let source = sources.source_by_name(src_ref.file.clone()).expect("source not found");
	let line_num = get_line_num(src_ref.start, &source);
	let underlined = underline_word(&src_ref, line_num, source, more);
	format!("line break\n{}\n", underlined)
}
fn debug_block(block:Block, sources:&Sources) -> String {
	let source = sources.source_by_name(block.opening.first_src_ref().file.clone()).expect("source not found");
	let o_src_ref = block.opening.first_src_ref();
	let c_src_ref = block.closing.first_src_ref();
	let o_line_num = get_line_num(o_src_ref.start, source);
	let c_line_num = get_line_num(c_src_ref.start, source);
	let o_underlined = underline_word(block.opening.first_src_ref(), o_line_num, source, None);
	let c_underlined = underline_word(block.closing.first_src_ref(), c_line_num, source, None);
	
	let mut inner = String::new();
	for t in block.tokens {
		inner.push('\t');
		inner.push_str(debug_token(t, sources, None).as_str());
	}
	
	format!("block on line {}\n{}\n{}\nclosing on line {}\n{}\n", o_line_num, o_underlined, inner, c_line_num, c_underlined)
}

fn underline_word(src_ref:&SourceRef, line_num:usize, source:&String, more:Option<&str>) -> String {
	let (line_start, line_end) = get_line(line_num, source);
	let line_num_as_string = line_num.to_string();
	let pre_line = format!("{} | ", line_num_as_string);
	let pre_indent = format!("{} | ", " ".repeat(line_num_as_string.len()));
	let line = &source[line_start..line_end];
	let indent_size = max(0, src_ref.start as isize - line_start as isize) as usize;
	let indent = " ".repeat(indent_size);
	let underline = "^".repeat(src_ref.end-src_ref.start);
	let more = more.unwrap_or("");
	format!("  --> {}:{}\n{}\n{}{}\n{}{}{} {}", src_ref.file, src_ref.start ,pre_indent, pre_line, line, pre_indent, indent, underline, more)
}

fn get_line_num(pos:usize, source:&String) -> usize {
	let mut line_num:usize = 1;
	for (i, c) in source.chars().enumerate() {
		if c == '\n' {
			if i > pos { break; }
			line_num += 1;
		}
	}
	line_num
}
fn get_line(line_num:usize, source:&String) -> (usize, usize) {
	let mut line_start = 0;
	let mut line_end:Option<usize> = None;
	let mut cline_num = 1;
	for (i, c) in source.chars().enumerate() {
		if c == '\n' {
			if cline_num == line_num {
				line_end = Some(i);
				break;
			}
			cline_num += 1;
			line_start = i+1;
		}
	}
	let line_end = line_end.unwrap_or(source.len());
	(line_start, line_end)
}
fn get_lines(first_line:usize, last_line:usize, source:&String) -> (usize, usize) {
	let start = get_line(first_line, source).0;
	let end = get_line(last_line, source).1;
	(start, end)
}
fn get_lines_as_str(first_line:usize, last_line:usize, source:&String) -> &str {
	let start = get_line(first_line, source).0;
	let end = get_line(last_line, source).1;
	&source[start..end]
}
