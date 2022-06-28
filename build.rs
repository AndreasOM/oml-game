use std::env;
use std::fs::File;
use std::path::Path;

use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn main() {
	let dest = env::var("OUT_DIR").unwrap();
	let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();

	Registry::new(Api::Gl, (4, 1), Profile::Core, Fallbacks::All, [])
		.write_bindings(GlobalGenerator, &mut file)
		.unwrap();
}
