use crate::math::{Vector2, Vector4};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub fn zero() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
		}
	}
	pub fn from_vector2(v: &Vector2) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: 0.0,
		}
	}

	pub fn from_vector4(v: &Vector4) -> Self {
		// :TODO: verify w==1.0 ??? or divide my w?

		Self {
			x: v.x,
			y: v.y,
			z: v.z,
		}
	}
}
