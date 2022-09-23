use std::sync::Arc;
use std::sync::Mutex;

use once_cell::sync::Lazy;

use crate::math::Matrix22;
use crate::math::Rectangle;
use crate::math::Vector2;
use crate::renderer::{Color, Renderer, SixteenSegment};

#[derive(Debug)]
struct Text {
	pos:   Vector2,
	text:  String,
	color: Color,
	scale: f32,
	width: f32,
}

#[derive(Debug)]
struct Line {
	start: Vector2,
	end:   Vector2,
	width: f32,
	color: Color,
}

#[derive(Debug)]
pub struct DebugRenderer {
	layer:  u8,
	effect: u16,
	offset: Vector2,
	lines:  Vec<Line>,
	texts:  Vec<Text>,
}

//pub static mut DEFAULT_DEBUGRENDERER: Option< Arc< Mutex< DebugRenderer > > > = None;
//pub static DEFAULT_DEBUGRENDERER: Arc< Mutex < Option< DebugRenderer > > > = Arc::new( Mutex::new( None ) );

/*
lazy_static! {
//    static ref ARRAY: Mutex<Vec<u8>> = Mutex::new(vec![]);
	pub static ref DEFAULT_DEBUGRENDERER: Arc< Mutex < Option< DebugRenderer > > > = Arc::new( Mutex::new( None ) );
}
*/

static DEFAULT_DEBUGRENDERER: Lazy<Arc<Mutex<Option<DebugRenderer>>>> =
	Lazy::new(|| Arc::new(Mutex::new(None)));

// :TODO: make these macros that compiles to NOP
pub fn debug_renderer_toggle(layer_id: u8, effect_id: u16) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if dr.is_none() {
			**dr = Some(DebugRenderer::new(layer_id, effect_id));
		} else {
			**dr = None;
		}
	}
}
pub fn debug_renderer_set_offset(offset: &Vector2) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.set_offset(offset);
		}
	}
}

pub fn debug_renderer_add_line(start: &Vector2, end: &Vector2, width: f32, color: &Color) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.add_line(start, end, width, color);
		}
	}
}
pub fn debug_renderer_add_rectangle(rect: &Rectangle, width: f32, color: &Color) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.add_rectangle(rect, width, color);
		}
	}
}

pub fn debug_renderer_add_frame(pos: &Vector2, size: &Vector2, width: f32, color: &Color) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.add_frame(pos, size, width, color);
		}
	}
}

pub fn debug_renderer_add_circle(pos: &Vector2, radius: f32, width: f32, color: &Color) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.add_circle(pos, radius, width, color);
		}
	}
}

pub fn debug_renderer_begin_frame() {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.begin_frame();
		}
	}
}

pub fn debug_renderer_render(renderer: &mut Renderer) {
	let mut lock = DEFAULT_DEBUGRENDERER.try_lock();
	if let Ok(ref mut dr) = lock {
		if let Some(dr) = &mut **dr {
			dr.render(renderer);
		}
	}
}

// end of macros

impl DebugRenderer {
	pub fn new(layer: u8, effect: u16) -> Self {
		Self {
			layer,
			effect,
			offset: Vector2::zero(),
			lines: Vec::new(),
			texts: Vec::new(),
		}
	}

	pub fn begin_frame(&mut self) {
		self.lines.clear();
		self.texts.clear();
	}
	pub fn end_frame(&mut self) {}

	pub fn set_offset(&mut self, offset: &Vector2) {
		self.offset = *offset;
	}

	fn render_line(
		&self,
		renderer: &mut Renderer,
		s: &Vector2,
		e: &Vector2,
		width: f32,
		color: &Color,
	) {
		let v0 = s;
		let v1 = e;
		let v01 = v1.sub(&v0).normalized();
		let vp = Vector2::new(-v01.y, v01.x);
		//			let vp = Vector2::new( 0.0, 1.0 );
		let vl = vp.scaled(0.5 * width);
		let vr = vp.scaled(-0.5 * width);

		let v0 = s.add(&vr);
		let v1 = e.add(&vr);

		let v2 = s.add(&vl);
		let v3 = e.add(&vl);

		//			println!("{:?} {:?} \n{:?} {:?} {:?} {:?} \n{:?} {:?} {:?} ",&l.start, &l.end, &v0,&v1,&v2, &v3, &v01, &vl, &vr);
		//			println!("{} + {} = {}", l.start.y, vl.y, v2.y );

		renderer.set_color(&color);

		let v0 = renderer.add_vertex(&v0);
		let v1 = renderer.add_vertex(&v1);
		let v2 = renderer.add_vertex(&v2);
		let v3 = renderer.add_vertex(&v3);

		renderer.add_triangle(v0, v1, v2);
		renderer.add_triangle(v2, v1, v3);
	}
	pub fn render(&self, renderer: &mut Renderer) {
		//		println!("Debug Render rendering");
		//		println!("{} lines", self.lines.len());

		renderer.use_layer(self.layer);
		renderer.use_effect(self.effect);
		for l in &self.lines {
			self.render_line(renderer, &l.start, &l.end, l.width, &l.color);
		}

		//		dbg!(&self.texts);
		for t in &self.texts {
			let scale = Vector2::new(t.scale, t.scale);
			let advance = Vector2::new(t.scale * (0.5 + 0.25), 0.0);
			let mut pos = t.pos;
			let l = t.text.len() as f32;
			pos.x -= advance.x * 0.5 * l;
			pos.y -= 0.5 * t.scale;
			for c in t.text.chars() {
				//				dbg!(&c);
				let lines = SixteenSegment::lines_for_character(c);
				//				dbg!(&lines);
				for (s, e) in lines {
					//					dbg!( &s, &e );
					let s = pos.add(&s.scaled_vector2(&scale));
					let e = pos.add(&e.scaled_vector2(&scale));

					self.render_line(renderer, &s, &e, t.width, &t.color);
				}
				pos = pos.add(&advance);
			}
		}

		//		todo!("die");
	}

	pub fn add_line(&mut self, start: &Vector2, end: &Vector2, width: f32, color: &Color) {
		let line = {
			Line {
				start: start.add(&self.offset),
				end: end.add(&self.offset),
				width,
				color: *color,
			}
		};
		self.lines.push(line);
	}
	pub fn add_text(&mut self, pos: &Vector2, text: &str, scale: f32, width: f32, color: &Color) {
		let text = Text {
			text: text.to_uppercase(),
			pos: *pos,
			color: *color,
			scale,
			width,
		};

		self.texts.push(text);
	}

	pub fn add_rectangle(&mut self, rect: &Rectangle, width: f32, color: &Color) {
		let s = &rect.bottom_left();
		let e = s.add(&rect.size());

		self.add_line(
			&Vector2::new(s.x, s.y),
			&Vector2::new(e.x, s.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(e.x, s.y),
			&Vector2::new(e.x, e.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(e.x, e.y),
			&Vector2::new(s.x, e.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(s.x, e.y),
			&Vector2::new(s.x, s.y),
			width,
			color,
		);
	}

	pub fn add_frame(&mut self, pos: &Vector2, size: &Vector2, width: f32, color: &Color) {
		let half_size = size.scaled_vector2(&Vector2::new(-0.5, 0.5));
		let top_left = pos.add(&half_size);
		let bottom_right = pos.sub(&half_size);

		self.add_line(&top_left, &bottom_right, width, color);
		self.add_line(
			&Vector2::new(bottom_right.x, top_left.y),
			&Vector2::new(top_left.x, bottom_right.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(top_left.x, top_left.y),
			&Vector2::new(top_left.x, bottom_right.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(bottom_right.x, top_left.y),
			&Vector2::new(bottom_right.x, bottom_right.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(top_left.x, top_left.y),
			&Vector2::new(bottom_right.x, top_left.y),
			width,
			color,
		);
		self.add_line(
			&Vector2::new(top_left.x, bottom_right.y),
			&Vector2::new(bottom_right.x, bottom_right.y),
			width,
			color,
		);
	}
	pub fn add_circle(&mut self, pos: &Vector2, radius: f32, width: f32, color: &Color) {
		let mut vr = Vector2::new(radius, 0.0);

		// we could add some code to decide on the number of segments here
		let n = 24;
		let r_step = 360.0 / n as f32;
		// rotate
		let mtx = Matrix22::z_rotation(r_step * 0.01745329252); // DEG to RAD

		let mut vertices = Vec::new();

		for _ in 0..n {
			let v = pos.add(&vr);
			vertices.push(v);

			vr = mtx.mul_vector2(&vr);
		}

		for i in 0..vertices.len() {
			let v0 = vertices[i];
			let v1 = vertices[(i + 1) % vertices.len()];
			self.add_line(&v0, &v1, width, color);
		}
	}
}
