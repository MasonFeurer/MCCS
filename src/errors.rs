use std::cmp::max;
use std::process::exit;
use crate::lexer::tokens::{Token, Scope};
use crate::bash_tools::*;

pub struct Help {
	pub message:String,
	pub source:String,
	pub line:usize,
}

pub fn missing_closing_brace(opening:Scope, at:usize) -> ! {
	let src = opening.source_file.get_source();
	
	println!("{}MCCS ERROR{}{}: missing closing brace on line {}{}", 
		color(BOLD, RED),
		RESET_COLOR,
		color(BOLD, LIGHT_WHITE),
		get_line(src, at),
		RESET_COLOR
	);
	println!("{}", underline_word(
		opening,
		Some("help: missing paired closing brace"),
		&color(PLAIN, CYAN), 
		true
	));
	exit(1);
}

pub fn illegal_token(token:Token, more:Option<&str>, help:Option<Help>) -> ! {
	println!("{}MCCS ERROR{}{}: illegal Token: {}{}", 
		color(BOLD, RED), 
		RESET_COLOR, 
		color(BOLD, LIGHT_WHITE), 
		token, 
		RESET_COLOR
	);
	
	let scope = token.get_scope();
	println!("{}", underline_word(scope, more, &color(BOLD, RED), true));
	if let Some(help) = help {
		println!("{}help{}: {}", 
			color(BOLD, CYAN),
			RESET_COLOR,
			help.message,
		);
		println!("{}", show_code(
			help.line,
			&help.source,
			&color(BOLD, GREEN),
		));
	}
	exit(1);
}


fn underline_word(scope:Scope, more:Option<&str>, underline:&String, show_file_loc:bool) -> String {
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
	let file_loc = if show_file_loc {
		format!("{}    -->{} {}:{}:{}\n", 
			color(BOLD, RED),
			RESET_COLOR,
			scope.source_file.path,
			line,
			scope.start-line_start,
		)
	}
	else { "".to_string() };
	
	format!("{}{}\n{}{}\n{}{}", 
		file_loc,
		gutter(None),
		gutter(Some(line)),
		line_src,
		gutter(None),
		underline_str
	)
}
fn show_code(line:usize, code:&String, color:&String) -> String {
	format!("{}\n{}{}{}{}\n{}", 
		gutter(None),
		gutter(Some(line)),
		color,
		code,
		RESET_COLOR,
		gutter(None),
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
