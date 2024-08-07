use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::sync::mpsc;

use backtrace::Backtrace;

pub mod debug_renderer;

//use crate::renderer::debug_renderer;

use crate::math::{
	Matrix22,
	Matrix32,
	//	Matrix33,
	Matrix44,
	Matrix44Stack,
	Vector2,
	Vector3,
};
use crate::renderer::debug_renderer::*;
use crate::system::System;
use crate::window::Window;

//use material::Material;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)] // clippy gives a false positive here
#[repr(C)]
pub struct Vertex {
	pos:        [f32; 3],
	tex_coords: [f32; 2],
	color:      [f32; 4],
}

impl Vertex {
	pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
		Self {
			pos:        [x, y, z],
			tex_coords: [0.0, 0.0],
			color:      [1.0, 1.0, 1.0, 1.0],
		}
	}
	pub fn from_pos_with_tex_coords(pos: &Vector2, tex_coords: &Vector2) -> Self {
		Self {
			pos:        [pos.x, pos.y, 0.0],
			tex_coords: [tex_coords.x, tex_coords.y],
			color:      [1.0, 1.0, 1.0, 1.0],
		}
	}
	pub fn from_pos_with_tex_coords_and_color(
		pos: &Vector2,
		tex_coords: &Vector2,
		color: &Color,
	) -> Self {
		Self {
			pos:        [pos.x, pos.y, 0.0],
			tex_coords: [tex_coords.x, tex_coords.y],
			color:      [color.r, color.g, color.b, color.a],
		}
	}
}

#[derive(Debug)]
enum Command {
	LoadTexture(u16, String),
}

#[derive(Debug)]
struct QueuedScreenshot {
	delay:    usize,          // how many frames to wait before starting
	filename: Option<String>, // template for the filename
	frames:   usize,          // number of frames to shoot
	taken:    usize,          // number of frames shot so far
}

#[derive(Debug, Default)]
struct ReadyScreenshot {
	filename: String,
	data:     Vec<u8>,
}

const MAX_TEXTURE_CHANNELS: usize = 4;
#[derive(Debug)]
pub struct Renderer {
	frame:             u64,
	material_manager:  Manager<Material>,
	texture_manager:   Manager<Texture>,
	font_manager:      FontManager,
	vertices:          Vec<Vertex>,
	effects:           HashMap<u16, Effect>,
	default_effect_id: u16,
	active_effect_id:  u16,
	active_layer_id:   u8,

	//	fonts: HashMap< u8, Font >,
	//	default_font_id: u8,
	active_font_id:   u8,
	active_font_name: String,

	active_textures: [Option<u16>; MAX_TEXTURE_CHANNELS],

	tex_coords: Vector2,
	color:      Color,

	mvp_matrix: Matrix44,
	tex_matrix: Matrix32,

	//	layer_matrix: [Option<Matrix44Stack>; 256],
	//	layer_matrix: Vec< Matrix44Stack >,
	layer_matrix: HashMap<u8, Matrix44Stack>,

	size:          Vector2,
	viewport_pos:  Vector2,
	viewport_size: Vector2,

	backtrace_on_missing: bool,
	// very tempted to move this whole logic into seperate struct
	command_rx:           Option<mpsc::Receiver<Command>>,
	command_tx:           Option<mpsc::Sender<Command>>,
	//textures_loading:     RwLock<HashSet<String>>,
	queued_screenshots:   Vec<QueuedScreenshot>,
	ready_screenshots:    VecDeque<ReadyScreenshot>,
}

impl Renderer {
	pub fn new() -> Self {
		Self {
			frame:             0,
			material_manager:  Manager::new(),
			texture_manager:   Manager::new(),
			font_manager:      FontManager::new(),
			vertices:          Vec::new(), // :TODO: pre allocate size? or maybe even a fixed size array
			effects:           HashMap::new(),
			//			fonts: HashMap::new(),
			default_effect_id: 0,
			active_effect_id:  0,
			active_layer_id:   0,
			//			default_font_id: 0,
			active_font_id:    0,
			active_font_name:  String::new(),
			active_textures:   [None; MAX_TEXTURE_CHANNELS],

			tex_coords: Vector2::zero(),
			color:      Color::white(),
			mvp_matrix: Matrix44::identity(),
			tex_matrix: Matrix32::identity(),

			//layer_matrix:  [None; 256],//[Matrix44Stack::default(); 256],
			//layer_matrix: Vec::with_capacity(256),
			layer_matrix: HashMap::new(),

			size:          Vector2::zero(),
			viewport_pos:  Vector2::zero(),
			viewport_size: Vector2::zero(),

			backtrace_on_missing: false,

			command_rx:         None,
			command_tx:         None,
			//textures_loading: RwLock::new(HashSet::new()),
			queued_screenshots: Vec::new(),
			ready_screenshots:  VecDeque::new(),
		}
	}

	pub fn update(&mut self, system: &mut System) {
		// :TODO: this could be running in a thread

		{
			let mut commands = VecDeque::new();

			if let Some(rx) = &self.command_rx {
				let mut commands_to_handle = 1000; // :TODO: tune for blocking vs rendering with wrong texture
				while commands_to_handle > 0 {
					match rx.try_recv() {
						Ok(cmd) => {
							commands.push_back(cmd);
							commands_to_handle -= 1;
						},
						Err(_e) => {
							commands_to_handle = 0;
						},
					}
				}
			}

			for cmd in commands {
				match cmd {
					Command::LoadTexture(depth, name) => {
						if depth < 16 {
							// :TODO: could be (almost) any limit
							match self.texture_manager.find_index(|t: &Texture| {
								//			dbg!(&t.name(), &name);
								t.name() == name
							}) {
								None => {
									println!(
										"[{:8}] Trying to load {} [depth {}]",
										self.frame, &name, depth
									);
									// try if it is a texture reference, aka .omtr
									let name_omtr = format!("{}.omtr", &name);
									let dfs = system.default_filesystem_mut();

									if dfs.exists(&name_omtr) {
										let mut f = dfs.open(&name_omtr);
										let mut line = Vec::new();

										while !f.eof() {
											let b = f.read_u8();
											if b == 0x0a || b == 0x0d {
												break;
											};

											line.push(b);
										}

										let line = String::from_utf8(line.clone()).unwrap();

										println!("\tFound reference! -> >{}<", &line);

										// just queue it as a command, to allow reference chains ... 0mg
										if let Some(tx) = &self.command_tx {
											let _ = tx.send(Command::LoadTexture(depth + 1, line));
										}
									} else {
										let cnt = TextureAtlas::load_all(system, self, &name);
										if cnt == 0 {
											println!("Warning: Tried to load atlas {}, but got no textures.", &name);
										}
										// :TODO: handle non atlas cases (not supported right now)
									}
								},
								Some(_i) => {
									// we already have it, so do nothing
									// :TODO: if we ever ref count textures this *might* be the place to increase it
								},
							};
						};
					},
				};
			}
			// save one ready screenshot
			if let Some(rs) = self.ready_screenshots.pop_front() {
				match self.save_screenshot(system, rs) {
					Ok(()) => {},
					Err(e) => {
						tracing::error!("Failed writing screenshot {}", e);
					},
				}
			}
		}
	}

	pub fn register_effect(&mut self, effect: Effect) {
		if self.effects.len() == 0 {
			self.default_effect_id = effect.id();
		}
		self.effects.insert(effect.id(), effect);
	}

	pub fn find_effect_mut_and_then<F>(&mut self, name: &str, mut f: F) -> bool
	where
		F: FnMut(&mut Effect),
	{
		self.effects
			.iter_mut()
			.find(|(_i, e)| e.name() == name)
			.map_or(false, |(_i, e)| {
				f(e);
				true
			})
	}

	pub fn register_texture(&mut self, texture: Texture) -> u16 {
		let index = self.texture_manager.add(texture);
		if self.texture_manager.len() == 1 {
			//			self.texture_manager.set_active( index );
			self.active_textures[0] = Some(index as u16);
		}
		index as u16
	}

	pub fn load_font(&mut self, system: &mut System, font_id: u8, name: &str) {
		let texture = Texture::create(system, name);
		let mut font = Font::create(system, name);
		font.recalc_from_matrix(texture.width());

		self.register_texture(texture);

		let _index = self.font_manager.add(font_id, font);
	}

	fn get_default_effect(&self) -> &Effect {
		match self.effects.get(&self.default_effect_id) {
			Some(e) => e,
			None => panic!("No default render Effect"),
		}
	}

	fn get_active_effect(&self) -> &Effect {
		match self.effects.get(&self.active_effect_id) {
			Some(e) => e,
			None => {
				println!(
					"No active render Effect found for {} -> using default",
					&self.active_effect_id
				);
				if self.backtrace_on_missing {
					let bt = Backtrace::new();
					println!("{:?}", bt);
				}
				self.get_default_effect()
			},
		}
	}
	/*
		fn get_default_font(&self) -> &Font {
			match self.fonts.get( &self.default_font_id ) {
				Some( e ) => e,
				None => panic!("No default render Font")
			}
		}

		fn get_active_font(&self) -> &Font {
			match self.fonts.get( &self.active_font_id ) {
				Some( e ) => e,
				None => {
					println!("No active render Font -> using default");
					self.get_default_font()
				}
			}
		}
	*/
	pub fn setup(&mut self, window: &Window, _system: &mut System) -> anyhow::Result<()> {
		gl::load_with(|s| {
			let a = window.get_proc_address(s) as *const _;
			// let a = 0 as *const _; // force nullptr for testing
			// tracing::debug!("Loading {} -> {:?}", s, a);
			a
		}); // :TODO: maybe use CFBundleGetFunctionPointerForName directly

		unsafe {
			let s = gl::GetString(gl::VERSION);
			let s = String::from_utf8(std::ffi::CStr::from_ptr(s as *const _).to_bytes().to_vec())?;
			println!("GL Version: {}", s);
		}

		// ensure we have one texture
		self.register_texture(Texture::create_canvas("[]", 2));

		// setup channels for async handling, e.g. texture loading
		let (tx, rx) = mpsc::channel();

		self.command_tx = Some(tx);
		self.command_rx = Some(rx);

		Ok(())
	}

	pub fn teardown(&mut self) {
		self.command_rx = None;
		self.command_tx = None;
	}

	pub fn begin_frame(&mut self) {
		self.vertices.clear();
		for material in self.material_manager.iter_mut() {
			material.clear();
		}
		// ensure we have at least one material, and it is active
		if self.material_manager.len() == 0 {
			let mut textures = Vec::new();
			for i in 0..MAX_TEXTURE_CHANNELS {
				let ti = self.active_textures[i].unwrap_or(0);
				textures.push(self.texture_manager.get(ti as usize).unwrap());
			}
			let m = Material::new(self.active_layer_id, &self.get_default_effect(), textures);
			let i = self.material_manager.add(m);
			self.material_manager.set_active(i);
		}
		//		let default_effect_name = self.default_effect_name.clone();
		//		self.use_effect( &default_effect_name );

		unsafe {
			let p = &self.viewport_pos;
			let s = &self.viewport_size;
			gl::Viewport(p.x as i32, p.y as i32, s.x as i32, s.y as i32);
			//			gl::Scissor( self.pos.x as i32, self.pos.y as i32, self.size.x as i32, self.size.y as i32 );
			//			gl::Enable( gl::SCISSOR_TEST );
		}

		self.color = Color::white();

		self.layer_matrix.clear();
		/*
		for m in self.layer_matrix.iter_mut() {
			m.clear();
		}
		*/
	}

	pub fn end_frame(&mut self) {
		let mut total_vertices = 0;
		let mut total_materials = 0;
		let mut total_materials_with_vertices = 0;

		let debug = self.frame % 500 == 0;
		// just to avoid ghost

		// moved to Effect
		/*
		unsafe {
			//			gl::Disable(gl::CULL_FACE);
			gl::Enable(gl::CULL_FACE);
			gl::Disable(gl::DEPTH_TEST);
			//			gl::PolygonMode( gl::FRONT_AND_BACK, gl::LINE );
		}
		*/
		//		println!("---");
		// :TODO: fix rendering order
		let mut material_indices = Vec::new();
		for i in 0..self.material_manager.len() {
			material_indices.push(i);
		}

		material_indices.sort_unstable_by(|a, b| {
			let a = self.material_manager.get(*a).unwrap().key();
			let b = self.material_manager.get(*b).unwrap().key();

			a.partial_cmp(&b).unwrap()
		});

		for t in self.texture_manager.iter_mut() {
			t.update();
		}

		for i in material_indices {
			let material = self.material_manager.get_mut(i).unwrap();

			//			println!("SortKey: 0x{:016X}", material.key() );
			// :TODO: ask material for effect
			let effect_id = material.effect_id();
			let e = match self.effects.get_mut(&effect_id) {
				Some(e) => e,
				None => match self.effects.get_mut(&self.default_effect_id) {
					Some(e) => e,
					None => panic!("No default render Effect"),
				},
			};
			material.set_mvp_matrix(&self.mvp_matrix);
			let vc = material.render(e);
			total_vertices += vc;
			total_materials += 1;
			if vc > 0 {
				total_materials_with_vertices += 1;
			}
			if debug {
				//				println!("Rendered {} vertices for material {:?} with effect {:?}", vc, &material, &e );
			}
		}

		// glFlush or glFinish
		unsafe {
			gl::Flush();
		}

		let screenshots = self.update_queued_screenshots();
		if !screenshots.is_empty() {
			// tracing::debug!("Screenshots: {:#?}", screenshots);
			for s in screenshots {
				let s = format!("{}-{:06}", s, self.frame);
				match self.take_screenshot(&s) {
					Ok(_) => {},
					Err(e) => {
						tracing::error!("Failed taking screenshot {}", e);
					},
				}
			}
		}

		if debug {
			//			dbg!(&self.material_manager);
			println!(
				"Render Stats: {} {} {}",
				total_vertices, total_materials_with_vertices, total_materials
			);
		}
		self.frame += 1;
	}

	// rendering functions

	pub fn clear(&mut self, color: &Color) {
		//		println!("clear with {:?}", &color );
		// glClearColor and glClear
		unsafe {
			gl::ClearColor(color.r, color.g, color.b, color.a);
			gl::Clear(gl::COLOR_BUFFER_BIT); // :TODO: clear other buffers?
		}
	}

	pub fn aspect_ratio(&self) -> f32 {
		self.size.x / self.size.y
	}

	pub fn size(&self) -> &Vector2 {
		&self.size
	}
	pub fn viewport_size(&self) -> &Vector2 {
		&self.viewport_size
	}
	pub fn viewport_pos(&self) -> &Vector2 {
		&self.viewport_pos
	}

	pub fn frame(&self) -> u64 {
		self.frame
	}
	pub fn set_size(&mut self, size: &Vector2) {
		self.size = *size;
	}

	pub fn set_viewport(&mut self, pos: &Vector2, size: &Vector2) {
		self.viewport_pos = *pos;
		self.viewport_size = *size;
	}

	pub fn set_mvp_matrix(&mut self, mvp_matrix: &Matrix44) {
		self.mvp_matrix = *mvp_matrix;
	}

	pub fn set_tex_matrix(&mut self, tex_matrix: &Matrix32) {
		self.tex_matrix = *tex_matrix;
	}

	pub fn set_uniform_float(&mut self, name: &str, value: f32) {
		// :HACK:
		let m = self.material_manager.get_mut_active();
		m.set_uniform(name, &Uniform::F32(value));
	}

	pub fn set_uniform_matrix44(&mut self, name: &str, value: Matrix44) {
		// :HACK:
		let m = self.material_manager.get_mut_active();
		m.set_uniform(name, &Uniform::MATRIX44(value));
	}

	fn switch_active_material_if_needed(&mut self) {
		//		println!("switch_active_material_if_needed active_effect_name {}", &self.active_effect_name);
		let lid = self.active_layer_id;
		let eid = self.get_active_effect().id();
		let mut textures = Vec::new();
		for i in 0..MAX_TEXTURE_CHANNELS {
			let ti = self.active_textures[i].unwrap_or(0);
			textures.push(self.texture_manager.get(ti as usize).unwrap());
		}
		let tids = textures
			.iter()
			.map(|&t| t.hwid())
			.collect::<Vec<_>>()
			.to_vec();
		let key = Material::calculate_key(lid, eid, &tids);
		let can_render = {
			let m = self.material_manager.get_active();
			m.can_render(key)
		};

		if !can_render {
			let found_material = self
				.material_manager
				.select_active(|m: &Material| m.can_render(key));
			if !found_material {
				/*
				println!(
					"Didn't find material for layer id {} effect id {} active_effect_id {}",
					lid,
					eid,
					&self.active_effect_id
				);
				*/
				let mut textures = Vec::new();
				for i in 0..MAX_TEXTURE_CHANNELS {
					let ti = self.active_textures[i].unwrap_or(0);
					textures.push(self.texture_manager.get(ti as usize).unwrap());
				}
				let m = Material::new(self.active_layer_id, &self.get_active_effect(), textures);
				let i = self.material_manager.add(m);
				self.material_manager.set_active(i);
			}
		}
	}
	pub fn use_effect(&mut self, effect_id: u16) {
		self.active_effect_id = effect_id;
		self.switch_active_material_if_needed();
	}

	pub fn use_layer(&mut self, layer_id: u8) {
		self.active_layer_id = layer_id;
		self.switch_active_material_if_needed();
	}

	pub fn use_texture(&mut self, name: &str) {
		self.use_texture_in_channel(name, 0);
		/*
				let current_active_texture = self.texture_manager.get_active();
				if name != current_active_texture.name() {
		//			println!("Switching active texture from {} to {}", &current_active_texture.name(), &name );

					let found_texture = self.texture_manager.select_active(|t: &Texture|{
						t.name() == name
					});

					if !found_texture {
						println!("Warning: Texture {} not found using default", &name);
						self.texture_manager.set_active( 0 );
					}
					self.switch_active_material_if_needed();
				}
				*/
	}

	pub fn use_font(&mut self, font_id: u8) {
		self.active_font_id = font_id;
	}

	pub fn disable_texture_for_channel(&mut self, channel: u8) {
		if self.active_textures[channel as usize].is_none() {
			// :TODO: is this ok?
			return;
		}

		self.active_textures[channel as usize] = None;

		self.switch_active_material_if_needed();
	}

	pub fn use_texture_id_in_channel(&mut self, tex_id: u16, channel: u8) {
		self.active_textures[channel as usize] = Some(tex_id);
		self.switch_active_material_if_needed();
	}
	pub fn use_texture_in_channel(&mut self, name: &str, channel: u8) {
		// :TODO: avoid changing texture when it is already active
		//		dbg!(&self.texture_manager);
		match self.texture_manager.find_index(|t: &Texture| {
			//			dbg!(&t.name(), &name);
			t.name() == name
		}) {
			None => {
				//todo!("Texture not found {}. User error?", &name),
				println!(
					"[{:8}] Texture {} not found, trying to load. Using default.",
					self.frame, &name
				);
				self.active_textures[channel as usize] = Some(0);
				if let Some(tx) = &self.command_tx {
					let _ = tx.send(Command::LoadTexture(0, name.to_string()));
				}
			},
			Some(i) => {
				self.active_textures[channel as usize] = Some(i as u16);
				self.switch_active_material_if_needed();
			},
		};
	}

	pub fn set_color(&mut self, color: &Color) {
		self.color = *color;
	}

	pub fn set_tex_coords(&mut self, tex_coords: &Vector2) {
		self.tex_coords = *tex_coords;
	}

	pub fn add_translation_for_layer(&mut self, layer_id: u8, offset: &Vector2) {
		// :TODO: use matrix stack
		let lm = self
			.layer_matrix
			.entry(layer_id)
			.or_insert(Matrix44Stack::default());
		let t = Matrix44::translation(&Vector3::from_vector2(&offset));
		lm.push_multiply(&t);
	}

	pub fn add_scaling_for_layer(&mut self, layer_id: u8, scaling: f32) {
		// :TODO: use matrix stack
		let lm = self
			.layer_matrix
			.entry(layer_id)
			.or_insert(Matrix44Stack::default());
		let s = Matrix44::scaling(scaling);
		lm.push_multiply(&s);
	}

	pub fn add_vertex(&mut self, pos: &Vector2) -> u32 {
		let lm = self
			.layer_matrix
			.entry(self.active_layer_id)
			.or_insert(Matrix44Stack::default());
		let m = lm.top();
		let pos = *m * *pos;
		let pos = &pos;
		// new
		let ti = self.active_textures[0].unwrap_or(0);
		let at = self
			.texture_manager
			.get(ti as usize)
			.unwrap_or(self.texture_manager.get(0).unwrap());

		let tex_mtx = *at.mtx();
		let user_tex_mtx = self.tex_matrix;
		// -- new
		let tc = &self.tex_coords;
		let tc = user_tex_mtx.mul_vector2(&tc);
		let tc = tex_mtx.mul_vector2(&tc);
		let v = Vertex::from_pos_with_tex_coords_and_color(pos, &tc, &self.color);
		self.vertices.push(v);
		self.vertices.len() as u32 - 1
	}

	pub fn add_triangle(&mut self, v0: u32, v1: u32, v2: u32) {
		let material = self.material_manager.get_mut_active();
		for v in [v0, v1, v2].iter() {
			match self.vertices.get(*v as usize) {
				Some(v) => {
					material.add_vertex(v);
				},
				None => {
					// :TODO: shout loud
				},
			}
		}
	}

	pub fn render_quad(&mut self, pos: &Vector2, size: &Vector2) {
		let mut hs = *size; // hs => half size
		hs.x = 0.5 * hs.x;
		hs.y = 0.5 * hs.y;

		let tl = Vector2::new(-hs.x + pos.x, hs.y + pos.y);
		let bl = Vector2::new(-hs.x + pos.x, -hs.y + pos.y);
		let br = Vector2::new(hs.x + pos.x, -hs.y + pos.y);
		let tr = Vector2::new(hs.x + pos.x, hs.y + pos.y);

		let v0 = self.add_vertex(&tl);
		let v1 = self.add_vertex(&bl);
		let v2 = self.add_vertex(&br);
		let v3 = self.add_vertex(&tr);

		self.add_triangle(v0, v1, v2); // TopLeft, BottomLeft, BottomRight
		self.add_triangle(v2, v3, v0); // BottomRight, TopRight, TopLeft
	}

	pub fn render_textured_fullscreen_quad(&mut self) {
		let size = self.size;
		//		dbg!(&size);
		self.render_textured_quad(&Vector2::zero(), &size);
	}

	pub fn render_textured_centered_fullheight_quad(&mut self, aspect: f32) {
		let our_aspect = self.size.x / self.size.y;

		let mut tex_mtx = Matrix32::identity();
		let pos = Vector2::zero();
		let mut size = *self.size();
		// let mut color = Color::red();
		let width = 3.0;
		if our_aspect > aspect {
			// centered, partial cover
			// color = Color::green();
			size.x = size.y * aspect;
		} else {
			// overlap
			// color = Color::blue();

			let oa = our_aspect / aspect;
			let tx = (1.0 - oa) * 0.5;
			//tracing::debug!("o: {}, a: {}, oa:{}, tx: {}", our_aspect, aspect, oa, tx);
			tex_mtx = tex_mtx
				.with_scaling_xy(oa, 1.0)
				.with_translation(&Vector2::new(tx, 0.0));
		}

		//debug_renderer_add_frame(&pos, &size, width, &color);

		self.set_tex_matrix(&tex_mtx);
		self.render_textured_quad(&pos, &size);
		self.set_tex_matrix(&Matrix32::identity());
	}

	pub fn render_textured_quad(&mut self, pos: &Vector2, size: &Vector2) {
		self.render_textured_quad_with_rotation(pos, size, 0.0);
	}

	pub fn render_textured_quad_with_rotation(
		&mut self,
		pos: &Vector2,
		size: &Vector2,
		angle: f32,
	) {
		let angle = angle * 0.01745329252;

		let mtx = Matrix22::z_rotation(angle);

		let positions = [
			Vector2::new(-0.5, 0.5),
			Vector2::new(-0.5, -0.5),
			Vector2::new(0.5, -0.5),
			Vector2::new(0.5, 0.5),
		];

		let tex_coords = [
			Vector2::new(0.0, 0.0),
			Vector2::new(0.0, 1.0),
			Vector2::new(1.0, 1.0),
			Vector2::new(1.0, 0.0),
		];

		//		let tex_mtx = Matrix32::identity();
		let ti = self.active_textures[0].unwrap_or(0);
		let at = self
			.texture_manager
			.get(ti as usize)
			.unwrap_or(self.texture_manager.get(0).unwrap());
		/*
				let tex_mtx = *at.mtx();
				let user_tex_mtx = self.tex_matrix;
		*/
		let mut v = [0u32; 4];

		// :TODO: future optimization once we have full matrix implementation
		//		let mtx_tr = Matrix32::translation( pos.x, pos.y );
		//		let mtx = mtx_r.mul_matrix( &mtx );
		for i in 0..4 {
			let p = positions[i];
			let p = p.scale_vector2(&size);
			let p = mtx.mul_vector2(&p).add(&pos);

			// this calculation has been moved into add_vertex
			let t = tex_coords[i];
			/*
			let t = user_tex_mtx.mul_vector2(&t);
			let t = tex_mtx.mul_vector2(&t);
			*/

			self.set_tex_coords(&t);
			v[i] = self.add_vertex(&p);
		}

		self.add_triangle(v[0], v[1], v[2]);
		self.add_triangle(v[2], v[3], v[0]);
	}
	/*
		pub fn render_textured_quad_with_tex_matrix( &mut self, pos: &Vector2, size: &Vector2, mtx: &Matrix32 ) {
	//		let mtx = Matrix22::z_rotation( angle );

			let positions = [
				Vector2::new( -0.5,  0.5 ),
				Vector2::new( -0.5, -0.5 ),
				Vector2::new(  0.5, -0.5 ),
				Vector2::new(  0.5,  0.5 ),
			];

			let tex_coords = [
				Vector2::new( 0.0, 0.0 ),
				Vector2::new( 0.0, 1.0 ),
				Vector2::new( 1.0, 1.0 ),
				Vector2::new( 1.0, 0.0 ),
			];

	//		let tex_mtx = Matrix32::identity();
			let ti = self.active_textures[ 0 ].unwrap_or( 0 );
			let at = self.texture_manager.get( ti as usize ).unwrap_or(
				self.texture_manager.get( 0 ).unwrap()
			);

	//		let tex_mtx = *at.mtx();
			let tex_mtx = mtx;	// :TODO: combine with active texture matrix - if we want fonts in atlases
			let user_tex_mtx = self.tex_matrix;

			let mut v = [0u32;4];

			// :TODO: future optimization once we have full matrix implementation
	//		let mtx_tr = Matrix32::translation( pos.x, pos.y );
	//		let mtx = mtx_r.mul_matrix( &mtx );
			for i in 0..4 {
				let p = positions[ i ];
				let p = p.scale_vector2( &size );
	//			let p = mtx.mul_vector2( &p ).add( &pos );

				// :TODO: decide if we might want to move this calculation to set_tex_coords
				let t = tex_coords[ i ];
				let t = user_tex_mtx.mul_vector2( &t );
				let t = tex_mtx.mul_vector2( &t );

				self.set_tex_coords( &t );
				v[ i ] = self.add_vertex( &p );
			}

			self.add_triangle( v[ 0 ], v[ 1 ], v[ 2 ] );
			self.add_triangle( v[ 2 ], v[ 3 ], v[ 0 ] );
		}
	*/
	pub fn find_texture_mut(&mut self, name: &str) -> Option<&mut Texture> {
		self.texture_manager.find_mut(|t| t.name() == name)
	}

	pub fn find_texture_mut_and_then<F>(&mut self, name: &str, mut f: F) -> bool
	where
		F: FnMut(&mut Texture),
	{
		self.texture_manager
			.find_mut(|t| t.name() == name)
			.map_or(false, |e| {
				f(e);
				true
			})
	}

	pub fn print(&mut self, pos: &Vector2, size: &Vector2, alignment: &Vector2, text: &str) {
		let old_texture_id = self.active_textures[0];
		{
			let font = self.font_manager.get(self.active_font_id);
			self.active_font_name = font.name().to_owned();
		}
		// :HACK:
		/*
		let font_name = match self.active_font_id {
					0 => "pink",
					_ => "pink_huge", // font.name();
		};
		*/
		let font_name = self.active_font_name.clone();
		self.use_texture(&font_name);

		let mut layout = TextLayout::new();
		{
			let font = self.font_manager.get(self.active_font_id);
			layout.layout(font, &Vector2::zero(), text);
		}

		//		let layout_pos = pos;	// :TODO: apply alignment
		let layout_size_half = layout.size().scaled(0.5);
		//		let layout_pos = pos.sub( &layout_size_half );
		//		let cpos       = ws.sub( &cs ).scaled( 0.5 ).scaled_vector2( &g );
		let layout_pos = size
			.sub(layout.size())
			.scaled(0.5)
			.scaled_vector2(alignment);
		let layout_pos = pos.add(&layout_pos);
		let layout_pos_2 = layout_pos.sub(&layout_size_half);

		for q in layout.quads() {
			self.set_tex_matrix(&q.tex_mtx);
			// :TODO: fix position for alignment
			self.render_textured_quad(&q.pos.add(&layout_pos_2), &q.size);
			//			self.render_textured_quad_with_tex_matrix( &q.pos, &q.size, &q.tex_mtx );
			debug_renderer::debug_renderer_add_frame(
				&q.pos.add(&layout_pos_2),
				&q.size,
				3.0,
				&Color::red(),
			);
		}

		//		dbg!(&layout);
		//		todo!("die");

		self.set_tex_matrix(&Matrix32::identity());
		self.active_textures[0] = old_texture_id;
		self.switch_active_material_if_needed();

		debug_renderer::debug_renderer_add_frame(
			&pos,
			&size,
			5.0,
			&Color::from_rgba(0.9, 0.75, 0.3, 0.6),
		);

		debug_renderer::debug_renderer_add_frame(
			&layout_pos,
			&layout.size(),
			3.0,
			&Color::from_rgba(0.4, 0.75, 0.3, 0.6),
		);
	}

	fn save_screenshot(
		&self,
		system: &mut System,
		ready_screenshots: ReadyScreenshot,
	) -> anyhow::Result<()> {
		let fs = system.savegame_filesystem_mut();
		if !fs.writable() {
			anyhow::bail!("Filesystem is not writable {:?}", fs);
		}
		let mut f = fs.create(&ready_screenshots.filename, false); // :TODO: overwrite?
		if !f.is_valid() {
			anyhow::bail!("couldn't write to {:?}", f);
		}

		tracing::debug!("Saving");
		let mut saved = 0;
		for b in ready_screenshots.data.iter() {
			f.write_u8(*b);
			saved += 1;
			if saved % 1024 == 0 {
				//				tracing::debug!("{} kb ({})", saved / 1024, saved );
			}
		}

		tracing::debug!("{:?}", system.savegame_filesystem_mut());
		tracing::debug!(
			"Wrote {}/{} bytes to {:?}",
			f.pos(),
			ready_screenshots.data.len(),
			f
		);

		Ok(())
	}
	fn take_screenshot(&mut self, filename: &str) -> anyhow::Result<()> {
		tracing::debug!("save_screenshot: {}", filename);
		let w = self.viewport_size.x as usize;
		let h = self.viewport_size.y as usize;
		let len = w * h * 4;
		let v: Vec<u8> = vec![0; len];
		let mut buffer = v.into_boxed_slice();
		unsafe {
			// :TODO:
			// gl::ReadBuffer
			// gl::PixelTransfer
			// gl::PixelMap
			gl::ReadPixels(
				0,
				0,
				w as i32,
				h as i32,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				buffer.as_mut_ptr() as *mut core::ffi::c_void,
			);
		}

		// flip buffer upside down
		// :TODO: we can make this far more efficient
		let mut buffer_flipped = vec![0; len];
		for y in 0..h {
			for x in 0..w {
				let sp = (w * y + x) * 4;
				let dp = (w * (h - y - 1) + x) * 4;
				buffer_flipped[dp] = buffer[sp];
				buffer_flipped[dp + 1] = buffer[sp + 1];
				buffer_flipped[dp + 2] = buffer[sp + 2];
				buffer_flipped[dp + 3] = buffer[sp + 3];
			}
		}

		let buffer = buffer_flipped;

		let mut png_buffer = Vec::new();
		{
			let mut encoder = png::Encoder::new(Cursor::new(&mut png_buffer), w as u32, h as u32);
			encoder.set_color(png::ColorType::Rgba);
			encoder.set_depth(png::BitDepth::Eight);

			encoder
				.add_itxt_chunk("Creator".to_string(), "oml-game".to_string())
				.unwrap();
			let mut writer = encoder.write_header().unwrap();
			writer.write_image_data(&buffer).unwrap();
		}

		if png_buffer[0] != 0x89 {
			anyhow::bail!("Broken header from PNG");
		} else {
			tracing::debug!("OK");
		}

		self.ready_screenshots.push_back(ReadyScreenshot {
			filename: format!("{}.png", filename),
			data:     png_buffer,
		});

		Ok(())
	}
	pub fn queue_screenshot(&mut self, delay: usize, frames: usize, filename: Option<&str>) {
		let qs = QueuedScreenshot {
			delay,
			frames,
			filename: filename.map(|s| s.to_owned()),
			taken: 0,
		};

		self.queued_screenshots.push(qs);
	}

	pub(crate) fn update_queued_screenshots(&mut self) -> Vec<String> {
		let mut r = Vec::new();
		let mut to_remove = Vec::new();
		for i in 0..self.queued_screenshots.len() {
			let mut qs = &mut self.queued_screenshots[i];
			qs.delay = qs.delay.saturating_sub(1);
			if qs.delay <= 0 {
				qs.taken += 1;
				if qs.taken >= qs.frames {
					to_remove.push(i);
				}
				let filename: String = qs
					.filename
					.as_ref()
					.unwrap_or(&"ScreenShot".to_string())
					.clone();
				r.push(filename);
			}
		}

		for i in to_remove {
			// self.queued_screenshots.swap_remove( i );
			self.queued_screenshots.remove(i);
		}

		r
	}
}

#[derive(Debug)]
struct Manager<T> {
	materials:    Vec<T>,
	active_index: usize,
}

#[allow(dead_code)]
impl<T> Manager<T> {
	pub fn new() -> Self {
		println!("Creating manager for {}", std::any::type_name::<T>());

		Self {
			materials:    Vec::new(),
			active_index: 0,
		}
	}

	pub fn set_active(&mut self, index: usize) {
		self.active_index = index;
	}

	pub fn select_active<F>(&mut self, f: F) -> bool
	where
		F: Fn(&T) -> bool,
	{
		for (i, m) in self.materials.iter().enumerate() {
			if f(m) {
				self.active_index = i;
				return true;
			}
		}
		false
	}

	pub fn len(&self) -> usize {
		self.materials.len()
	}
	pub fn add(&mut self, material: T) -> usize {
		let i = self.materials.len();
		self.materials.push(material);
		i
	}

	pub fn get(&self, index: usize) -> Option<&T> {
		self.materials.get(index)
	}

	pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
		self.materials.get_mut(index)
	}

	pub fn find_index<F>(&mut self, f: F) -> Option<usize>
	where
		F: Fn(&T) -> bool,
	{
		for (i, m) in self.materials.iter().enumerate() {
			if f(m) {
				return Some(i);
			}
		}
		None
	}

	pub fn find_mut<F>(&mut self, f: F) -> Option<&mut T>
	where
		F: Fn(&T) -> bool,
	{
		for m in self.materials.iter_mut() {
			if f(&m) {
				return Some(m);
			}
		}

		None
	}
	pub fn iter(&mut self) -> std::slice::Iter<'_, T> {
		self.materials.iter()
	}
	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
		self.materials.iter_mut()
	}
	pub fn get_mut_active(&mut self) -> &mut T {
		match self.materials.get_mut(self.active_index) {
			Some(m) => m,
			None => panic!("No active {}", std::any::type_name::<T>()),
		}
	}
	pub fn get_active(&self) -> &T {
		match self.materials.get(self.active_index) {
			Some(m) => m,
			None => panic!("No active {}", std::any::type_name::<T>()),
		}
	}
}

#[derive(Debug)]
struct FontManager {
	fonts: HashMap<u8, Font>,
}

impl FontManager {
	pub fn new() -> Self {
		Self {
			fonts: HashMap::new(),
		}
	}

	pub fn add(&mut self, id: u8, font: Font) {
		self.fonts.insert(id, font);
	}

	pub fn get(&self, id: u8) -> &Font {
		self.fonts.get(&id).unwrap()
	}
}

mod animated_texture;
pub use animated_texture::AnimatedTexture;
pub use animated_texture::AnimatedTextureConfiguration;

mod blend_factor;
pub use blend_factor::BlendFactor;

mod debug;
pub use debug::Debug;
mod gl {
	include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod color;
pub use color::Color;
mod effect;
pub use effect::Effect;
mod font;
pub use font::Font;
mod material;
pub use material::Material;
//mod material_builder;
//	pub use material_builder::MaterialBuilder as MaterialBuilder;
mod program;
pub use program::Program;
pub use program::ShaderType;
mod text_layout;
pub use text_layout::TextLayout;
mod texture;
pub use texture::Texture;
mod texture_atlas;
pub use texture_atlas::TextureAtlas;
mod uniform;
pub use uniform::Uniform;

mod sixteen_segment;
pub use sixteen_segment::SixteenSegment;
