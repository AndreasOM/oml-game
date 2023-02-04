#[derive(Debug, Default)]
pub enum BlendFactor {
	Zero,
	#[default]
	One,
	DstColor,
	OneMinusDstColor,
	SrcAlpha,
	OneMinusSrcAlpha,
	DstAlpha,
	OneMinusDstAlpha,
	SrcAlphaSaturate,
	// Not supported yet
	ConstantColor,
	OneMinusConstantColor,
	ConstantAlpha,
	OneMinusConstantAlpha,
}

use crate::renderer::gl;
impl From<BlendFactor> for gl::types::GLenum {
	fn from(bf: BlendFactor) -> Self {
		match bf {
			BlendFactor::Zero => gl::ZERO,
			BlendFactor::One => gl::ONE,
			BlendFactor::DstColor => gl::DST_COLOR,
			BlendFactor::OneMinusDstColor => gl::ONE_MINUS_DST_COLOR,
			BlendFactor::SrcAlpha => gl::SRC_ALPHA,
			BlendFactor::OneMinusSrcAlpha => gl::ONE_MINUS_SRC_ALPHA,
			BlendFactor::OneMinusDstAlpha => gl::ONE_MINUS_DST_ALPHA,
			BlendFactor::SrcAlphaSaturate => gl::SRC_ALPHA_SATURATE,
			o => {
				tracing::warn!("{:?} not mapped to GLenum", o);
				gl::ONE
			},
		}
	}
}
