use std::fmt::{Display, Formatter};

pub struct SourceRef {
	pub file:String,
	pub start:usize,
	pub end:usize,
}
impl Display for SourceRef {
	fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
		f.write_str(format!("{{file '{}', {}->{}}}", self.file, self.start, self.end).as_str())
	}
}

pub struct Sources {
	pub files: Vec<(String, String)>,
}
impl Sources {
	pub fn new() -> Sources {
		Sources { files:Vec::new() }
	}
	pub fn push_source(&mut self, file_name:String, source:String) {
		self.files.push((file_name, source));
	}
	pub fn source_by_name(&self, name:String) -> Option<&String> {
		for f in &self.files {
			if f.0 == name {
				return Some(&f.1);
			}
		}
		None
	}
}
