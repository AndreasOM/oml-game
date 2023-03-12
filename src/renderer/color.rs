//rgb(244, 67, 54)
use once_cell::sync::Lazy;
static PAL1: Lazy<Vec<Color>> = Lazy::new(|| {
	//let mut i = 0.0;
	//let s = 0.01;
	let pal = [
		/* :TODO: experiment with rotating Hsv colors
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
			Color::from_hsv({i+=s;i*1.0},0.5,0.8),
		*/
		Color::from_rgba_u8(244, 67, 54, 255),
		Color::from_rgba_u8(33, 150, 243, 255),
		Color::from_rgba_u8(139, 195, 74, 255),
		Color::from_rgba_u8(121, 85, 72, 255),
		Color::from_rgba_u8(156, 39, 176, 255),
		Color::from_rgba_u8(0, 188, 212, 255),
		Color::from_rgba_u8(255, 235, 59, 255),
		Color::from_rgba_u8(0, 150, 136, 255),
		Color::from_rgba_u8(255, 152, 0, 255),
	]
	.to_vec();
	pal
});
static mut PAL1_INDEX: usize = 0;

#[derive(Debug, Copy, Clone)]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

impl Default for Color {
	fn default() -> Self {
		Color::white()
	}
}
/*
static PAL1: &[Color] = &[
	Color::from_rgba_u8(244, 67, 54, 255 ),
];
*/

impl Color {
	pub fn pal(index: usize) -> &'static Self {
		let index = index % PAL1.len();
		unsafe {
			PAL1_INDEX = index + 1;
		}
		&PAL1[index]
	}
	pub fn pal_next() -> &'static Self {
		unsafe {
			let p = Self::pal(PAL1_INDEX);
			PAL1_INDEX += 1;
			p
		}
	}
	pub fn white() -> Self {
		Self {
			r: 1.0,
			g: 1.0,
			b: 1.0,
			a: 1.0,
		}
	}

	pub fn black() -> Self {
		Self {
			r: 0.0,
			g: 0.0,
			b: 0.0,
			a: 1.0,
		}
	}

	pub fn red() -> Self {
		Self {
			r: 1.0,
			g: 0.0,
			b: 0.0,
			a: 1.0,
		}
	}
	pub fn green() -> Self {
		Self {
			r: 0.0,
			g: 1.0,
			b: 0.0,
			a: 1.0,
		}
	}
	pub fn blue() -> Self {
		Self {
			r: 0.0,
			g: 0.0,
			b: 1.0,
			a: 1.0,
		}
	}
	pub fn rainbow(t: f32) -> Self {
		Self::from_hsv(t, 1.0, 1.0)
	}
	pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
		Self { r, g, b, a }
	}
	pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: a as f32 / 255.0,
		}
	}

	pub fn from_a(a: f32) -> Self {
		Self {
			r: a,
			g: a,
			b: a,
			a,
		}
	}

	pub fn from_hsv(h: f32, s: f32, v: f32) -> Self {
		let a = 1.0;

		let (r, g, b) = if s <= 0.0 {
			// zero saturation, pure grey
			(v, v, v)
		} else {
			let mut hh = h % 360.0;
			hh /= 60.0;

			let i = hh.floor() as usize; // which of the 6 segments?
			let ff = hh - i as f32; // fraction in segment

			let p = v * (1.0 - s);
			let q = v * (1.0 - (s * ff));
			let t = v * (1.0 - (s * (1.0 - ff)));

			match i {
				0 => (v, t, p),
				1 => (q, v, p),
				2 => (p, v, t),
				3 => (p, q, v),
				4 => (t, p, v),
				5 | _ => (v, p, q),
			}
		};

		Self { r, g, b, a }
	}

	pub fn as_rgba8(&self) -> u32 {
		let r = (self.r * 255.0) as u32;
		let g = (self.g * 255.0) as u32;
		let b = (self.b * 255.0) as u32;
		let a = (self.a * 255.0) as u32;
		(r << 24) | (g << 16) | (b << 8) | (a << 0)
	}
	pub fn as_abgr8(&self) -> u32 {
		let r = (self.r * 255.0) as u32;
		let g = (self.g * 255.0) as u32;
		let b = (self.b * 255.0) as u32;
		let a = (self.a * 255.0) as u32;
		(r << 0) | (g << 8) | (b << 16) | (a << 24)
	}
}

impl core::ops::Mul for Color {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		Self {
			r: self.r * rhs.r,
			g: self.g * rhs.g,
			b: self.b * rhs.b,
			a: self.a * rhs.a,
		}
	}
}
