use std::cmp::max;
use std::process::exit;
use crate::lexer::{Token, Scope};
use crate::bash_tools::*;

pub fn missing_closing_brace(opening:&Scope, at:usize) -> ! {
	let src = opening.source_file.get_source();
	
	println!("{}MCCS ERROR{}{}: Missing closing brace on line {}{}", 
		color(BOLD, RED),
		RESET_COLOR,
		color(BOLD, LIGHT_WHITE),
		get_line(src, at),
		RESET_COLOR
	);
	println!("{}", underline_word(
		opening,
		Some("help: missing paired closing brace"),
		&color(PLAIN, CYAN)
	));
	exit(1);
}

pub fn illegal_token(token:Token, more:Option<&str>) -> ! {
	println!("{}MCCS ERROR{}{}: Illegal Token: {}{}", 
		color(BOLD, RED), 
		RESET_COLOR, 
		color(BOLD, LIGHT_WHITE), 
		token, 
		RESET_COLOR
	);
	
	let scope = token.get_scope();
	println!("{}", underline_word(scope, more, &color(BOLD, RED)));
	exit(1);
}


fn underline_word(scope:&Scope, more:Option<&str>, underline:&String) -> String {
	let src = scope.source_file.get_source();
	let line = get_line(src, scope.start);
	let (line_start, line_end) = get_line_src(src, line);
	let line_src = &src[line_start..line_end];
	let indent_size = max(0, scope.start as isize - line_start as isize) as usize;
	let underline_str = format!("{}{}{} {}{}", 
		underline,
		" ".repeat(indent_size),
		"^".repeat(scope.end-scope.start),
		more.unwrap_or(""),
		RESET_COLOR
	);
	format!("{}    -->{} {}:{}:{}\n{}\n{}{}\n{}{}", 
		color(BOLD, RED),
		RESET_COLOR,
		scope.source_file.path,
		line,
		scope.start-line_start,
		gutter(None),
		gutter(Some(line)),
		line_src,
		gutter(None),
		underline_str
	)
}
fn gutter(line:Option<usize>) -> String {
	match line {
		Some(line) => {
			format!("{}{:>5} | {}", 
				color(BOLD, BLUE),
				line,
				RESET_COLOR
			)
		}
		None => {
			format!("{}      | {}", 
				color(BOLD, BLUE),
				RESET_COLOR
			)
		}
	}
}

fn get_line(src:&String, pos:usize) -> usize {
	let mut line:usize = 1;
	for (i, c) in src.chars().enumerate() {
		if c == '\n' {
			if i > pos { break; }
			line += 1;
		}
	}
	line
}
fn get_line_src(src:&String, line:usize) -> (usize, usize) {
	let mut line_start = 0;
	let mut line_end:Option<usize> = None;
	let mut cline = 1;
	for (i, c) in src.chars().enumerate() {
		if c == '\n' {
			if cline == line {
				line_end = Some(i);
				break;
			}
			cline += 1;
			line_start = i+1;
		}
	}
	let line_end = line_end.unwrap_or(src.len());
	(line_start, line_end)
}
