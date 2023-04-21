use std::ops::Rem;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::math::Vector4;

#[derive(Default, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct Vector2 {
	pub x: f32,
	pub y: f32,
}

impl Vector2 {
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn from_x_str(x_str: &str) -> Self {
		let size: Vec<f32> = x_str
			.split("x")
			.map(|s| f32::from_str(s.trim()).unwrap_or(0.0))
			.collect();
		Self {
			x: size[0],
			y: size[1],
		}
	}
	pub fn zero() -> Self {
		Self { x: 0.0, y: 0.0 }
	}

	pub fn from_vector4(v: &Vector4) -> Self {
		// :TODO: danger, ignore w?
		Self { x: v.x, y: v.y }
	}

	pub fn normalized(&self) -> Self {
		let l = self.length();
		Self {
			x: self.x / l,
			y: self.y / l,
		}
	}

	pub fn reciprocal(&self) -> Self {
		Self {
			x: 1.0 / self.x,
			y: 1.0 / self.y,
		}
	}

	pub fn scaled(&self, factor: f32) -> Self {
		Self {
			x: self.x * factor,
			y: self.y * factor,
		}
	}

	pub fn cross(&self, other: &Vector2) -> Self {
		Self {
			x: self.y * other.x - self.x * other.y,
			y: self.x * other.y - self.y * other.x,
		}
	}

	pub fn dot(&self, other: &Vector2) -> f32 {
		self.x * other.x + self.y * other.y
	}

	// :TODO: seems to be duplicated from scale_vector2
	pub fn scaled_vector2(&self, factor: &Vector2) -> Self {
		Self {
			x: self.x * factor.x,
			y: self.y * factor.y,
		}
	}

	pub fn scaled_reciprocal_vector2(&self, factor: &Vector2) -> Self {
		Self {
			x: self.x / factor.x,
			y: self.y / factor.y,
		}
	}

	pub fn add(&self, o: &Vector2) -> Self {
		Self {
			x: self.x + o.x,
			y: self.y + o.y,
		}
	}

	pub fn sub(&self, o: &Vector2) -> Self {
		Self {
			x: self.x - o.x,
			y: self.y - o.y,
		}
	}

	pub fn length(&self) -> f32 {
		let sql = self.x * self.x + self.y * self.y;
		sql.sqrt()
	}

	// r = atan2(length(cross(a,b)), dot(a,b))
	// return 360 - (Mathf.Atan2(vector2.x, vector2.y) * Mathf.Rad2Deg * Mathf.Sign(vector2.x));
	pub fn angle(&self) -> f32 {
		//let r = self.y.atan2( self.x );
		//		let r = self.x.atan2( self.y ) * self.x.signum();
		let r = self.y.atan2(self.x); // * self.y.signum();
		let d = 180.0 * r / std::f32::consts::PI;

		let d = d + 360.0;
		let d = d.rem(360.0);
		d
	}

	pub fn scale_vector2(&self, o: &Vector2) -> Self {
		Self {
			x: self.x * o.x,
			y: self.y * o.y,
		}
	}
}

impl From<(f32, f32)> for Vector2 {
	fn from(t: (f32, f32)) -> Self {
		Self { x: t.0, y: t.1 }
	}
}

impl From<(f64, f64)> for Vector2 {
	fn from(t: (f64, f64)) -> Self {
		Self {
			x: t.0 as f32,
			y: t.1 as f32,
		}
	}
}

impl std::fmt::Debug for Vector2 {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "Vector2: [{}, {}]", self.x, self.y,)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn from_x_str_works() -> anyhow::Result<()> {
		let v = Vector2::from_x_str("64x64");
		assert_eq!(64.0, v.x);
		assert_eq!(64.0, v.y);

		let v = Vector2::from_x_str("      64   x    64   ");
		assert_eq!(64.0, v.x);
		assert_eq!(64.0, v.y);

		Ok(())
	}
}
