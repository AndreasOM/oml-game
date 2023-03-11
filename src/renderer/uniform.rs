use crate::math::Matrix44;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum Uniform {
	F32(f32),
	MATRIX44(Matrix44),
}
