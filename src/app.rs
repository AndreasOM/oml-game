use crate::window::{Window, WindowUpdateContext};

pub trait App {
	fn setup(&mut self, _window: &mut Window) -> anyhow::Result<()> {
		Ok(())
	}
	fn teardown(&mut self) {}
	fn is_done(&self) -> bool {
		true
	}
	fn update(&mut self, _wuc: &mut WindowUpdateContext) -> anyhow::Result<()> {
		Ok(())
	}
	fn fixed_update(&mut self, _time_step: f64) {}
	fn render(&mut self) {}

	fn remember_window_layout(&self) -> bool {
		false
	}

	fn app_name(&self) -> &str {
		"oml-game"
	}
	fn layout_filename(&self) -> String {
		format!("{}_layout.yaml", self.app_name())
	}
}
