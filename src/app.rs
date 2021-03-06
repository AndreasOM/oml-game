use crate::window::{Window, WindowUpdateContext};

pub trait App {
	fn setup(&mut self, _window: &mut Window) -> anyhow::Result<()> {
		Ok(())
	}
	fn teardown(&mut self) {}
	fn is_done(&self) -> bool {
		true
	}
	fn update(&mut self, _wuc: &mut WindowUpdateContext) {}
	fn render(&mut self) {}
}
