use crate::renderer::{
	gl,
	//	Debug,
	BlendFactor,
	Program,
	ShaderType,
};
use crate::system::System;

#[derive(Debug)]
pub struct Effect {
	id: u16,
	name: String,
	program: Program,
	cull_face: bool,
	depth_test: bool,
	blend_source_factor: gl::types::GLenum,
	blend_destination_factor: gl::types::GLenum,
}

/*
	gl::Enable(gl::CULL_FACE);
	gl::Disable(gl::DEPTH_TEST);
*/
impl Effect {
	pub fn create(
		system: &mut System,
		id: u16,
		name: &str,
		vertex_shader_name: &str,
		fragment_shader_name: &str,
	) -> Self {
		Effect::new(system, id, name, vertex_shader_name, fragment_shader_name)
	}
	fn new(
		system: &mut System,
		id: u16,
		name: &str,
		vertex_shader_name: &str,
		fragment_shader_name: &str,
	) -> Self {
		let mut program = Program::new();

		let mut vsf = system.default_filesystem_mut().open(vertex_shader_name);
		let vs = vsf.read_as_string();

		let mut fsf = system.default_filesystem_mut().open(fragment_shader_name);
		let fs = fsf.read_as_string();

		program.add_shader(ShaderType::Vertex, &vs);
		program.add_shader(ShaderType::Fragment, &fs);
		program.link();

		Self {
			id,
			name: name.to_string(),
			program,
			cull_face: true,
			depth_test: false,
			blend_source_factor: gl::SRC_ALPHA,
			blend_destination_factor: gl::ONE_MINUS_SRC_ALPHA,
		}
	}

	pub fn id(&self) -> u16 {
		self.id
	}

	pub fn r#use(&mut self) {
		unsafe {
			if self.cull_face {
				gl::Enable(gl::CULL_FACE);
			} else {
				gl::Disable(gl::CULL_FACE);
			}
			if self.depth_test {
				gl::Enable(gl::DEPTH_TEST);
			} else {
				gl::Disable(gl::DEPTH_TEST);
			}

			gl::BlendFunc(self.blend_source_factor, self.blend_destination_factor);
		}

		self.program.r#use();
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn program(&self) -> &Program {
		&self.program
	}

	pub fn with_cull_face(mut self, cull_face: bool) -> Self {
		self.cull_face = cull_face;
		self
	}
	pub fn set_cull_face(&mut self, cull_face: bool) {
		self.cull_face = cull_face;
	}
	pub fn with_depth_test(mut self, depth_test: bool) -> Self {
		self.depth_test = depth_test;
		self
	}
	pub fn with_blend_func(
		mut self,
		source_factor: BlendFactor,
		destination_factor: BlendFactor,
	) -> Self {
		self.blend_source_factor = source_factor.into();
		self.blend_destination_factor = destination_factor.into();
		self
	}

	pub fn set_blend_func(&mut self, source_factor: BlendFactor, destination_factor: BlendFactor) {
		self.blend_source_factor = source_factor.into();
		self.blend_destination_factor = destination_factor.into();
	}
}
