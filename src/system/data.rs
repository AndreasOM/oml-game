pub trait Data {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		writeln!(f, "[Trait] Data")
	}
}

impl std::fmt::Debug for dyn Data {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		self.fmt(f)
	}
}
