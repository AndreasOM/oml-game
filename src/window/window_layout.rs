use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::math::Vector2;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WindowLayoutWindowConfig {
	pos:  Vector2,
	size: Vector2,
}

impl WindowLayoutWindowConfig {
	pub fn pos(&self) -> &Vector2 {
		&self.pos
	}
	pub fn size(&self) -> &Vector2 {
		&self.size
	}
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WindowLayout {
	windows: HashMap<String, WindowLayoutWindowConfig>,
}

impl WindowLayout {
	pub fn load(&mut self, filename: &Path) -> anyhow::Result<()> {
		let f = std::fs::File::open(&filename)?;
		let c = serde_yaml::from_reader(&f)?;
		*self = c;

		//		dbg!(&self);
		Ok(())
	}

	pub fn save(&self, filename: &Path) -> anyhow::Result<()> {
		let s = serde_yaml::to_string(&self)?;
		//		dbg!(&s);
		//		let mut buffer = File::create( filename )?;
		//		buffer.write_all( &s.as_bytes() );
		std::fs::write(&filename, &s.as_bytes())?;
		//		write!(buffer, &s);
		Ok(())
	}

	pub fn set_window(&mut self, name: &str, pos: &Vector2, size: &Vector2) {
		let mut window = self
			.windows
			.entry(name.to_string())
			.or_insert(WindowLayoutWindowConfig::default());
		window.pos = *pos;
		window.size = *size;
	}

	pub fn get_window(&self, name: &str) -> Option<&WindowLayoutWindowConfig> {
		self.windows.get(name)
	}
}
