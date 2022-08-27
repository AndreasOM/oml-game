use derive_getters::Getters;

use crate::math::Vector2;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum Anchor {
	#[default]
	BottomLeft,
	Center,
}

#[derive(Debug, Default, Copy, Clone, Getters)]
pub struct Rectangle {
	bottom_left: Vector2,
	center:      Vector2,
	size:        Vector2,
	anchor:      Anchor,
}

impl Rectangle {
	pub fn x(&self) -> f32 {
		match self.anchor {
			Anchor::BottomLeft => self.bottom_left.x,
			Anchor::Center => self.center.x,
		}
	}
	pub fn y(&self) -> f32 {
		match self.anchor {
			Anchor::BottomLeft => self.bottom_left.y,
			Anchor::Center => self.center.y,
		}
	}
	pub fn width(&self) -> f32 {
		self.size.x
	}
	pub fn height(&self) -> f32 {
		self.size.y
	}
	/* :DEPRECATED:
		pub fn set_x(&mut self, x: f32) {
			self.bottom_left.x = x;
		}
		pub fn set_y(&mut self, y: f32) {
			self.bottom_left.y = y;
		}
		pub fn set_width(&mut self, width: f32) {
			self.size.x = width;
		}
		pub fn set_height(&mut self, height: f32) {
			self.size.y = height;
		}
	*/
	pub fn offset(&mut self, offset: &Vector2) {
		self.bottom_left = self.bottom_left.add(offset);
	}

	pub fn with_offset(mut self, offset: &Vector2) -> Self {
		self.bottom_left = self.bottom_left.add(offset);
		self
	}

	pub fn with_center(mut self, center: &Vector2) -> Self {
		self.anchor = Anchor::Center;
		self.center = *center;

		self.recalc_from_center();
		self
	}

	fn recalc_from_center(&mut self) {
		self.bottom_left = self.center.add(&self.size.scaled(-0.5));
	}

	fn recalc_from_bottom_left(&mut self) {
		self.center = self.bottom_left.add(&self.size.scaled(0.5));
	}

	pub fn with_bottom_left(mut self, bottom_left: &Vector2) -> Self {
		self.anchor = Anchor::BottomLeft;
		self.bottom_left = *bottom_left;

		self.recalc_from_bottom_left();
		self
	}

	pub fn with_size(mut self, size: &Vector2) -> Self {
		self.size = *size;
		match self.anchor {
			Anchor::BottomLeft => self.recalc_from_bottom_left(),
			Anchor::Center => self.recalc_from_center(),
		};
		self
	}

	pub fn hflip(&mut self, pivot: f32) {
		match self.anchor {
			Anchor::BottomLeft => {
				//rect.set_y(pivot_y - pos.y - size.y);
				self.bottom_left.y = pivot - self.bottom_left.y - self.size.y;
				self.recalc_from_bottom_left();
			},
			a => todo!("Implement hflip for anchor {:?}", a),
		};
	}
	/* :DEPRECATED:
		pub fn with_x(mut self, x: f32) -> Self {
			self.bottom_left.x = x;
			self
		}
		pub fn with_y(mut self, y: f32) -> Self {
			self.bottom_left.y = y;
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
	*/
}

impl From<(f32, f32, f32, f32)> for Rectangle {
	fn from(t: (f32, f32, f32, f32)) -> Self {
		let bottom_left = Vector2::new(t.0, t.1);
		let size = Vector2::new(t.2, t.3);
		Self::default()
			.with_bottom_left(&bottom_left)
			.with_size(&size)
	}
}

impl From<(f64, f64, f64, f64)> for Rectangle {
	fn from(t: (f64, f64, f64, f64)) -> Self {
		let bottom_left = Vector2::new(t.0 as f32, t.1 as f32);
		let size = Vector2::new(t.2 as f32, t.3 as f32);
		Self::default()
			.with_bottom_left(&bottom_left)
			.with_size(&size)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/* :DEPRECATED:
		#[test]
		fn can_position_and_size() {
			let r = Rectangle::default()
				.with_x(1.0)
				.with_y(2.0)
				.with_width(3.0)
				.with_height(4.0);

			assert_eq!(r.bottom_left().x, 1.0);
			assert_eq!(r.bottom_left().y, 2.0);
			assert_eq!(r.size().x, 3.0);
			assert_eq!(r.size().y, 4.0);
			assert_eq!(r.x(), 1.0);
			assert_eq!(r.y(), 2.0);
			assert_eq!(r.width(), 3.0);
			assert_eq!(r.height(), 4.0);
		}
	*/
	#[test]
	fn can_position_and_size_with_vector2() {
		let r = Rectangle::default()
			.with_bottom_left(&Vector2::new(1.0, 2.0))
			.with_size(&Vector2::new(3.0, 4.0));

		assert_eq!(r.bottom_left().x, 1.0);
		assert_eq!(r.bottom_left().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}

	#[test]
	fn can_position_and_size_from_f64_tuple() {
		let r: Rectangle = (1.0f64, 2.0, 3.0, 4.0).into();

		assert_eq!(r.bottom_left().x, 1.0);
		assert_eq!(r.bottom_left().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}

	#[test]
	fn can_position_and_size_from_f32_tuple() {
		let r: Rectangle = (1.0f32, 2.0, 3.0, 4.0).into();

		assert_eq!(r.bottom_left().x, 1.0);
		assert_eq!(r.bottom_left().y, 2.0);
		assert_eq!(r.size().x, 3.0);
		assert_eq!(r.size().y, 4.0);
		assert_eq!(r.x(), 1.0);
		assert_eq!(r.y(), 2.0);
		assert_eq!(r.width(), 3.0);
		assert_eq!(r.height(), 4.0);
	}
	#[test]
	fn can_position_and_size_with_bottom_left() {
		let r: Rectangle = Rectangle::default()
			.with_bottom_left(&Vector2::new(-5.0, -10.0))
			.with_size(&Vector2::new(10.0, 20.0));
		assert_eq!(r.x(), -5.0);
		assert_eq!(r.y(), -10.0);
		assert_eq!(r.width(), 10.0);
		assert_eq!(r.height(), 20.0);
		assert_eq!(r.center().x, 0.0);
		assert_eq!(r.center().y, 0.0);
		assert_eq!(r.bottom_left().x, -5.0);
		assert_eq!(r.bottom_left().y, -10.0);
	}

	#[test]
	fn can_position_and_size_with_center() {
		let r: Rectangle = Rectangle::default()
			.with_center(&Vector2::new(0.0, 0.0))
			.with_size(&Vector2::new(10.0, 20.0));
		assert_eq!(r.x(), 0.0);
		assert_eq!(r.y(), 0.0);
		assert_eq!(r.width(), 10.0);
		assert_eq!(r.height(), 20.0);
		assert_eq!(r.center().x, 0.0);
		assert_eq!(r.center().y, 0.0);
		assert_eq!(r.bottom_left().x, -5.0);
		assert_eq!(r.bottom_left().y, -10.0);
	}
}
