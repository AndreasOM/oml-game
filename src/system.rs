use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::system::filesystem::Filesystem;
use crate::system::filesystem_empty::FilesystemEmpty;
//use crate::system::filesystem_stream::FilesystemStream;

#[derive(Debug)]
pub struct System {
	default_filesystem:  Box<dyn Filesystem>,
	savegame_filesystem: Box<dyn Filesystem>,
	data:                Option<Arc<dyn Data>>,
}

impl Default for System {
	fn default() -> Self {
		Self {
			default_filesystem:  Box::new(FilesystemEmpty::new()),
			savegame_filesystem: Box::new(FilesystemEmpty::new()),
			data:                None,
		}
	}
}

impl System {
	pub fn new() -> Self {
		Self {
			..Default::default()
		}
	}

	pub fn set_default_filesystem(&mut self, fs: Box<dyn Filesystem>) {
		self.default_filesystem = fs;
	}

	pub fn default_filesystem_mut(&mut self) -> &mut Box<dyn Filesystem> {
		&mut self.default_filesystem
	}

	pub fn default_filesystem(&self) -> &Box<dyn Filesystem> {
		&self.default_filesystem
	}

	pub fn set_savegame_filesystem(&mut self, fs: Box<dyn Filesystem>) {
		self.savegame_filesystem = fs;
	}

	pub fn savegame_filesystem_mut(&mut self) -> &mut Box<dyn Filesystem> {
		&mut self.savegame_filesystem
	}

	pub fn set_data(&mut self, data: Arc<dyn Data>) {
		if self.data.is_some() {
			panic!(
				"Tried to set_data for System more than once, this is probably not want you want!"
			)
		}
		self.data = Some(data);
	}

	pub fn data(&self) -> &Option<Arc<dyn Data>> {
		&self.data
	}

	pub fn get_document_dir(name: &str) -> String {
		let doc_dir = dirs_next::document_dir().unwrap();
		let doc_dir = doc_dir.to_string_lossy();
		let dir = format!("{}/{}", doc_dir, name);

		fs::create_dir_all(&dir).unwrap();
		dir
	}

	pub fn get_resource_path(name: &str) -> Option<String> {
		let exe_dir = std::env::current_exe().unwrap();
		let exe_path = Path::new(&exe_dir).parent().unwrap();

		//		dbg!(&exe_path);

		let p = exe_path.join("../Resources").join(&name);
		//		dbg!(&p);
		if p.exists() {
			return Some(p.to_string_lossy().to_string());
		}

		let p = exe_path.join(&name);
		//		dbg!(&p);
		if p.exists() {
			return Some(p.to_string_lossy().to_string());
		}

		let p = exe_path.join("../..").join(&name);
		//		dbg!(&p);
		if p.exists() {
			return Some(p.to_string_lossy().to_string());
		}
		None
	}
}

pub mod filesystem;
pub mod filesystem_stream;

pub mod filesystem_archive;
pub mod filesystem_stream_archive;

pub mod filesystem_disk;
pub mod filesystem_stream_disk;

pub mod filesystem_empty;
pub mod filesystem_stream_empty;

pub mod filesystem_memory;
pub mod filesystem_stream_memory;

pub mod filesystem_layered;

mod serializer;
pub use serializer::Serializer;

pub mod audio_fileloader_system;

mod data;
pub use data::Data;

#[cfg(test)]
mod tests {
	use oml_audio::Audio;
	use oml_audio::AudioBackend;

	use super::*;
	#[test]
	fn oml_audio_via_system_works() -> anyhow::Result<()> {
		let mut audio: Box<dyn AudioBackend<System>> = Audio::create_default();
		audio.start();
		audio.update();
		eprintln!("{:?}", audio.backend_type());

		Ok(())
	}
}
