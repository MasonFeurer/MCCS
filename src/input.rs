#[derive(Clone, Debug)]
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
