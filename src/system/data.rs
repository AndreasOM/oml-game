pub trait Data {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl std::fmt::Debug for dyn Data {
	fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		todo!()
	}
}

pub struct DataEmpty {}

impl DataEmpty {
	pub fn new() -> Self {
		Self {}
	}
}
impl Data for DataEmpty {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}
