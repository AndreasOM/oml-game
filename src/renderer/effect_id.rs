
pub trait EffectId {
	fn as_u16() -> u16 where Self: Sized;
}


pub trait EffectIdFactory {
	fn create( name: &str ) -> Box<dyn EffectId>;
}
