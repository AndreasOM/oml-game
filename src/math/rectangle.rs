use derive_getters::Getters;

use crate::math::Vector2;

#[derive(Debug, Default, Copy, Clone, Getters)]
pub struct Rectangle {
	pos:  Vector2,
	size: Vector2,
}

impl Rectangle {
	pub fn x(&self) -> f32 {
		self.pos.x
	}
	pub fn y(&self) -> f32 {
		self.pos.y
	}
	pub fn width(&self) -> f32 {
		self.size.x
	}
	pub fn height(&self) -> f32 {
		self.size.y
	}

	pub fn set_x(&mut self, x: f32) {
		self.pos.x = x;
	}
	pub fn set_y(&mut self, y: f32) {
		self.pos.y = y;
	}
	pub fn set_width(&mut self, width: f32) {
		self.size.x = width;
	}
	pub fn set_height(&mut self, height: f32) {
		self.pos.y = height;
	}

	pub fn offset(&mut self, offset: &Vector2) {
		self.pos = self.pos.add(offset);
	}

	pub fn with_offset(mut self, offset: &Vector2) -> Self {
		self.pos = self.pos.add(offset);
		self
	}

	pub fn with_pos(mut self, pos: &Vector2) -> Self {
		self.pos = *pos;
		self
	}

	pub fn with_size(mut self, size: &Vector2) -> Self {
		self.size = *size;
		self
	}

	pub fn with_x(mut self, x: f32) -> Self {
		self.pos.x = x;
		self
	}
	pub fn with_y(mut self, y: f32) -> Self {
		self.pos.y = y;
		self
	}
	pub fn with_width(mut self, width: f32) -> Self {
		self.size.x = width;
		self
	}
	pub fn with_height(mut self, height: f32) -> Self {
		self.size.y = height;
		self
	}
}

impl From<(f32, f32, f32, f32)> for Rectangle {
	fn from(t: (f32, f32, f32, f32)) -> Self {
		Self {
			pos:  Vector2::new(t.0, t.1),
			size: Vector2::new(t.2, t.3),
		}
	}
}

impl From<(f64, f64, f64, f64)> for Rectangle {
	fn from(t: (f64, f64, f64, f64)) -> Self {
		Self {
			pos:  Vector2::new(t.0 as f32, t.1 as f32),
			size: Vector2::new(t.2 as f32, t.3 as f32),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_position_and_size() {
		let r = Rectangle::default()
			.with_x(1.0)
			.with_y(2.0)
			.with_width(3.0)
			.with_height(4.0);

		assert_eq!(r.pos().x, 1.0);
		assert_eq!(r.pos().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}

	#[test]
	fn can_position_and_size_with_vector2() {
		let r = Rectangle::default()
			.with_pos(&Vector2::new(1.0, 2.0))
			.with_size(&Vector2::new(3.0, 4.0));

		assert_eq!(r.pos().x, 1.0);
		assert_eq!(r.pos().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}

	#[test]
	fn can_position_and_size_from_f64_tuple() {
		let r: Rectangle = (1.0, 2.0, 3.0, 4.0).into();

		assert_eq!(r.pos().x, 1.0);
		assert_eq!(r.pos().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}

	#[test]
	fn can_position_and_size_from_f32_tuple() {
		let r: Rectangle = (1.0, 2.0, 3.0, 4.0).into();

		assert_eq!(r.pos().x, 1.0);
		assert_eq!(r.pos().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}
}
