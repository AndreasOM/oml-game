use crate::math::Matrix44;

#[derive(Debug, Default)]
pub struct Matrix44Stack {
	stack: Vec<Matrix44>,
	top:   Matrix44,
}

impl Matrix44Stack {
	pub fn top(&self) -> &Matrix44 {
		&self.top
	}

	pub fn push(&mut self, m: &Matrix44) {
		let t = std::mem::replace(&mut self.top, *m);
		self.stack.push(t);
	}

	pub fn push_multiply(&mut self, m: &Matrix44) {
		let m = self.top.multiply(m);
		let t = std::mem::replace(&mut self.top, m);
		self.stack.push(t);
	}

	pub fn pop(&mut self) {
		let t = match self.stack.pop() {
			Some(t) => t,
			_ => panic!("Tried to pop from empty Matrix44Stack"),
		};

		let t = std::mem::replace(&mut self.top, t);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn push_and_pop_works() -> anyhow::Result<()> {
		let mut ms = Matrix44Stack::default();

		let i = Matrix44::identity();
		let z = Matrix44::zero();

		let t = ms.top();
		assert_eq!(i, *t);

		ms.push(&Matrix44::zero());

		let t = ms.top();
		assert_eq!(z, *t);

		// test the mul
		let m0 = Matrix44::new(
			0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
		);
		let m1 = Matrix44::new(
			0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
		);
		let me = Matrix44::new(
			56.0, 62.0, 68.0, 74.0, 152.0, 174.0, 196.0, 218.0, 248.0, 286.0, 324.0, 362.0, 344.0,
			398.0, 452.0, 506.0,
		);

		ms.push(&m0);
		ms.push_multiply(&m1);
		let t = ms.top();
		assert_eq!(me, *t);

		ms.pop();
		ms.pop();

		ms.pop();
		let t = ms.top();
		assert_eq!(i, *t);

		Ok(())
	}
}
