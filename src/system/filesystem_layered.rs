use crate::system::filesystem::Filesystem;
use crate::system::filesystem_stream::FilesystemStream;

#[derive(Debug)]
pub struct FilesystemLayered {
	filesystems: Vec<Box<dyn Filesystem>>,
}

impl FilesystemLayered {
	pub fn new() -> Self {
		Self {
			filesystems: Vec::new(),
		}
	}

	pub fn add_filesystem(&mut self, filesystem: Box<dyn Filesystem>) {
		self.filesystems.push(filesystem);
	}
}

impl Filesystem for FilesystemLayered {
	fn open(&mut self, name: &str) -> Box<dyn FilesystemStream> {
		for fs in self.filesystems.iter_mut().rev() {
			if fs.exists(name) {
				//				println!("{} exists in {:?}", &name, &fs);
				return fs.open(name);
			}
		}

		if let Some(fs) = self.filesystems.get_mut(0) {
			//			todo!("File not found: {}", name );
			fs.open(name)
		} else {
			panic!("Error: FilesystemLayered tried to open a file without any filesystem");
		}
	}

	fn create(&mut self, name: &str, overwrite: bool) -> Box<dyn FilesystemStream> {
		for fs in self.filesystems.iter_mut().rev() {
			if fs.writable() {
				let fss = fs.create(name, overwrite);
				if fss.is_valid() {
					return fss;
				}
			}
		}

		if let Some(fs) = self.filesystems.get_mut(0) {
			//			todo!("File not found: {}", name );
			fs.create(name, overwrite)
		} else {
			panic!("Error: FilesystemLayered tried to create a file without any filesystem");
		}
	}

	fn exists(&self, name: &str) -> bool {
		for fs in self.filesystems.iter().rev() {
			if fs.exists(name) {
				return true;
			}
		}
		//		dbg!(&self, &name);
		false
	}

	fn writable(&self) -> bool {
		for fs in self.filesystems.iter() {
			if fs.writable() {
				return true;
			}
		}
		false
	}

	fn name(&self) -> &str {
		""
	}

	fn filesystem_type(&self) -> &str {
		"Layered"
	}
}
