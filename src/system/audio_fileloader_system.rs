pub use oml_audio::fileloader::{FileLoader, FileLoaderFile};

use crate::system::filesystem_stream::FilesystemStream;
use crate::system::System;

pub struct FileLoaderFileForStream {
	stream: Box<dyn FilesystemStream>,
}

impl FileLoaderFileForStream {
	pub fn new(stream: Box<dyn FilesystemStream>) -> Self {
		Self { stream }
	}
}
impl FileLoader for System {
	fn open(&mut self, filename: &str) -> Box<dyn FileLoaderFile> {
		Box::new(FileLoaderFileForStream::new(
			self.default_filesystem_mut().open(&filename),
		))
	}
	fn exists(&self, filename: &str) -> bool {
		self.default_filesystem().exists(filename)
	}
}

impl FileLoaderFile for FileLoaderFileForStream {
	fn is_valid(&self) -> bool {
		self.stream.is_valid()
	}

	fn read_u8(&mut self) -> u8 {
		self.stream.read_u8()
	}

	fn eof(&self) -> bool {
		self.stream.eof()
	}

	fn name(&self) -> &str {
		self.stream.name()
	}
	fn pos(&self) -> usize {
		self.stream.pos()
	}

	fn set_pos(&mut self, pos: usize) -> usize {
		self.stream.set_pos(pos);
		self.stream.pos()
	}
}
