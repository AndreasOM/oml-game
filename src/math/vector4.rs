use crate::math::{Vector2, Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vector4 {
	pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
		Self { x, y, z, w }
	}

	pub fn from_vector2(v: &Vector2) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: 0.0,
			w: 1.0,
		}
	}

	pub fn from_vector3(v: &Vector3) -> Self {
		Self {
			x: v.x,
			y: v.y,
			z: v.z,
			w: 1.0,
		}
	}

	pub fn zero() -> Self {
		Self {
			x: 0.0,
			y: 0.0,
			z: 0.0,
			w: 0.0,
		}
	}
}
