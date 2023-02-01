use core::ops::Mul;
use std::ops::Index;

use crate::math::Vector2;
use crate::math::Vector3;
use crate::math::Vector4;

#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Matrix44 {
	m: [f32; 16],
}

impl Matrix44 {
	pub fn zero() -> Self {
		Self { m: [0.0; 16] }
	}

	pub fn new(
		v00: f32,
		v01: f32,
		v02: f32,
		v03: f32,
		v04: f32,
		v05: f32,
		v06: f32,
		v07: f32,
		v08: f32,
		v09: f32,
		v10: f32,
		v11: f32,
		v12: f32,
		v13: f32,
		v14: f32,
		v15: f32,
	) -> Self {
		Self {
			m: [
				v00, v01, v02, v03, v04, v05, v06, v07, v08, v09, v10, v11, v12, v13, v14, v15,
			],
		}
	}

	pub fn identity() -> Self {
		Self {
			m: [
				1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
			],
		}
	}

	pub fn translation(v: &Vector3) -> Self {
		Self {
			m: [
				1.0, 0.0, 0.0, v.x, 0.0, 1.0, 0.0, v.y, 0.0, 0.0, 1.0, v.z, 0.0, 0.0, 0.0, 1.0,
			],
		}
	}

	pub fn scaling(s: f32) -> Self {
		Self {
			m: [
				s, 0.0, 0.0, 0.0, 0.0, s, 0.0, 0.0, 0.0, 0.0, s, 0.0, 0.0, 0.0, 0.0, 1.0,
			],
		}
	}

	pub fn multiply_vector4(&self, rhs: &Vector4) -> Vector4 {
		let x = rhs.x;
		let y = rhs.y;
		let z = rhs.z;
		let w = rhs.w;

		Vector4::new(
			x * self.m[0 * 4 + 0]
				+ y * self.m[0 * 4 + 1]
				+ z * self.m[0 * 4 + 2]
				+ w * self.m[0 * 4 + 3],
			x * self.m[1 * 4 + 0]
				+ y * self.m[1 * 4 + 1]
				+ z * self.m[1 * 4 + 2]
				+ w * self.m[1 * 4 + 3],
			x * self.m[2 * 4 + 0]
				+ y * self.m[2 * 4 + 1]
				+ z * self.m[2 * 4 + 2]
				+ w * self.m[2 * 4 + 3],
			x * self.m[3 * 4 + 0]
				+ y * self.m[3 * 4 + 1]
				+ z * self.m[3 * 4 + 2]
				+ w * self.m[3 * 4 + 3],
		)
	}

	pub fn multiply(&self, rhs: &Self) -> Self {
		let a11 = self.m[0 * 4 + 0];
		let a12 = self.m[0 * 4 + 1];
		let a13 = self.m[0 * 4 + 2];
		let a14 = self.m[0 * 4 + 3];
		let a21 = self.m[1 * 4 + 0];
		let a22 = self.m[1 * 4 + 1];
		let a23 = self.m[1 * 4 + 2];
		let a24 = self.m[1 * 4 + 3];
		let a31 = self.m[2 * 4 + 0];
		let a32 = self.m[2 * 4 + 1];
		let a33 = self.m[2 * 4 + 2];
		let a34 = self.m[2 * 4 + 3];
		let a41 = self.m[3 * 4 + 0];
		let a42 = self.m[3 * 4 + 1];
		let a43 = self.m[3 * 4 + 2];
		let a44 = self.m[3 * 4 + 3];
		let b11 = rhs.m[0 * 4 + 0];
		let b12 = rhs.m[0 * 4 + 1];
		let b13 = rhs.m[0 * 4 + 2];
		let b14 = rhs.m[0 * 4 + 3];
		let b21 = rhs.m[1 * 4 + 0];
		let b22 = rhs.m[1 * 4 + 1];
		let b23 = rhs.m[1 * 4 + 2];
		let b24 = rhs.m[1 * 4 + 3];
		let b31 = rhs.m[2 * 4 + 0];
		let b32 = rhs.m[2 * 4 + 1];
		let b33 = rhs.m[2 * 4 + 2];
		let b34 = rhs.m[2 * 4 + 3];
		let b41 = rhs.m[3 * 4 + 0];
		let b42 = rhs.m[3 * 4 + 1];
		let b43 = rhs.m[3 * 4 + 2];
		let b44 = rhs.m[3 * 4 + 3];
		Self {
			m: [
				(a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41),
				(a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42),
				(a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43),
				(a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44),
				(a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41),
				(a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42),
				(a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43),
				(a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44),
				(a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41),
				(a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42),
				(a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43),
				(a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44),
				(a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41),
				(a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42),
				(a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43),
				(a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44),
			],
		}
	}

	pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
		let rpl = right + left;
		let rml = right - left;
		let tpb = top + bottom;
		let tmb = top - bottom;
		let fpn = far + near;
		let fmn = far - near;

		Self {
			m: [
				2.0 / rml,
				0.0,
				0.0,
				0.0,
				0.0,
				2.0 / tmb,
				0.0,
				0.0,
				0.0,
				0.0,
				2.0 / fmn,
				0.0,
				-rpl / rml,
				-tpb / tmb,
				-fpn / fmn,
				1.0,
			],
		}
	}

	pub fn as_ptr(&self) -> *const f32 {
		self.m.as_ptr()
	}
}

impl Default for Matrix44 {
	fn default() -> Self {
		Matrix44::identity()
	}
}

impl std::fmt::Debug for Matrix44 {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let m = &self.m;
		writeln!(
			f,
			"Matrix44:\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}\n{} {} {} {}",
			m[0],
			m[1],
			m[2],
			m[3],
			m[4],
			m[5],
			m[6],
			m[7],
			m[8],
			m[9],
			m[10],
			m[11],
			m[12],
			m[13],
			m[14],
			m[15],
		)
	}
}

impl Index<usize> for Matrix44 {
	type Output = f32;

	fn index(&self, index: usize) -> &Self::Output {
		&self.m[index]
	}
}

impl Mul<Matrix44> for Matrix44 {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self {
		self.multiply(&rhs)
	}
}

impl Mul<Vector4> for Matrix44 {
	type Output = Vector4;

	fn mul(self, rhs: Vector4) -> Vector4 {
		self.multiply_vector4(&rhs)
	}
}

impl Mul<Vector3> for Matrix44 {
	type Output = Vector3;

	fn mul(self, rhs: Vector3) -> Vector3 {
		let v4 = Vector4::from_vector3(&rhs);
		let vr4 = self.multiply_vector4(&v4);
		Vector3::from_vector4(&vr4)
	}
}

impl Mul<Vector2> for Matrix44 {
	type Output = Vector2;

	fn mul(self, rhs: Vector2) -> Vector2 {
		let v4 = Vector4::from_vector2(&rhs);
		let vr4 = self.multiply_vector4(&v4);
		Vector2::from_vector4(&vr4)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn access_works() -> anyhow::Result<()> {
		let m = Matrix44::new(
			0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
		);
		assert_eq!(0.0, m[0]);
		assert_eq!(1.0, m[1]);
		assert_eq!(2.0, m[2]);
		assert_eq!(3.0, m[3]);
		assert_eq!(4.0, m[4]);
		assert_eq!(5.0, m[5]);
		assert_eq!(6.0, m[6]);
		assert_eq!(7.0, m[7]);
		assert_eq!(8.0, m[8]);
		assert_eq!(9.0, m[9]);
		assert_eq!(10.0, m[10]);
		assert_eq!(11.0, m[11]);
		assert_eq!(12.0, m[12]);
		assert_eq!(13.0, m[13]);
		assert_eq!(14.0, m[14]);
		assert_eq!(15.0, m[15]);
		Ok(())
	}
	#[test]
	#[should_panic(expected = "index out of bounds: the len is 16 but the index is 16")]
	fn out_of_bounds_access_panics() {
		let m = Matrix44::new(
			0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
		);
		assert_eq!(16.0, m[16]);
	}

	#[test]
	fn multiplication_works() -> anyhow::Result<()> {
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

		//		let mr = m0.multiply(&m1);
		let mr = m0 * m1;
		assert_eq!(mr[0], me[0]);
		assert_eq!(mr[1], me[1]);
		assert_eq!(mr[2], me[2]);
		assert_eq!(mr[3], me[3]);
		assert_eq!(mr[4], me[4]);
		assert_eq!(mr[5], me[5]);
		assert_eq!(mr[6], me[6]);
		assert_eq!(mr[7], me[7]);
		assert_eq!(mr[8], me[8]);
		assert_eq!(mr[9], me[9]);
		assert_eq!(mr[10], me[10]);
		assert_eq!(mr[11], me[11]);
		assert_eq!(mr[12], me[12]);
		assert_eq!(mr[13], me[13]);
		assert_eq!(mr[14], me[14]);
		assert_eq!(mr[15], me[15]);

		assert_eq!(mr, me);

		let v0 = Vector4::new(0.0, 1.0, 2.0, 3.0);
		let vr = m0 * v0;
		let ve = Vector4::new(14.0, 38.0, 62.0, 86.0);

		assert_eq!(vr, ve);

		Ok(())
	}

	#[test]
	fn translation_works() -> anyhow::Result<()> {
		let v0 = Vector3::new(1.0, 2.0, 3.0);
		let v1 = Vector3::new(2.0, 3.0, 4.0);
		let ve = Vector3::new(3.0, 5.0, 7.0);

		let t = Matrix44::translation(&v1);
		let vr3 = t * v0;
		//		let vr3 = Vector3::from_vector4( &vr4 );

		assert_eq!(vr3, ve);

		Ok(())
	}
}
