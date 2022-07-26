use derive_getters::Getters;
use crate::math::Vector2;

#[derive(Debug, Default, Getters)]
pub struct Rectangle {
	pos: Vector2,
	size: Vector2,
}

impl Rectangle {

	pub fn x( &self ) -> f32 {
		self.pos.x
	}
	pub fn y( &self ) -> f32 {
		self.pos.y
	}
	pub fn width( &self ) -> f32 {
		self.size.x
	}
	pub fn height( &self ) -> f32 {
		self.size.y
	}

	pub fn set_pos( mut self, pos: &Vector2 ) -> Self {
		self.pos = *pos;
		self
	}

	pub fn set_size( mut self, size: &Vector2 ) -> Self {
		self.size = *size;
		self
	}

	pub fn set_x( mut self, x: f32 ) -> Self {
		self.pos.x = x;
		self
	}
	pub fn set_y( mut self, y: f32 ) -> Self {
		self.pos.y = y;
		self
	}
	pub fn set_width( mut self, width: f32 ) -> Self {
		self.size.x = width;
		self
	}
	pub fn set_height( mut self, height: f32 ) -> Self {
		self.size.y = height;
		self
	}
}


#[cfg(test)]
mod tests {
	use super::*;

		#[test]
	fn can_position_and_size() {
		let r = Rectangle::default()
					.set_x( 1.0 )
					.set_y( 2.0 )
					.set_width( 3.0 )
					.set_height( 4.0 )
				;

		assert_eq!(r.pos().x, 1.0);
		assert_eq!(r.pos().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}

	fn can_position_and_size_with_vector2() {
		let r = Rectangle::default()
					.set_pos( &Vector2::new( 1.0, 2.0 ) )
					.set_size( &Vector2::new( 3.0, 4.0 ) )
				;

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
