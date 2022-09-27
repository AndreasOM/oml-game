use crate::math::Vector2;

#[derive(Debug, Default)]

pub struct Circle {
	radius: f32,
	center: Vector2,
}

impl Circle {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn center(&self) -> &Vector2 {
		&self.center
	}

	pub fn radius(&self) -> f32 {
		self.radius
	}

	pub fn with_center(mut self, center: &Vector2) -> Self {
		self.center = *center;
		self
	}

	pub fn with_radius(mut self, radius: f32) -> Self {
		self.radius = radius;
		self
	}

	pub fn overlaps(&self, other: &Circle) -> bool {
		let d = self.center.sub(&other.center);
		let l = d.length();
		let r = self.radius + other.radius;

		r > l
	}
}
