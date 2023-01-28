use std::borrow::Cow;

use regex::Regex;

use crate::renderer::{Renderer, Texture};
use crate::system::System;

#[derive(Debug)]
pub struct AnimatedTextureConfiguration {
	pub template:    String,
	pub first_frame: u16,
	pub last_frame:  u16,
	pub fps:         f32,
}

impl AnimatedTextureConfiguration {
	pub fn new(
		template: &str,
		//		number_of_digits: i8,
		first_frame: u16,
		last_frame: u16,
		fps: f32,
	) -> Self {
		Self {
			template: template.to_owned(),
			first_frame,
			last_frame,
			fps,
		}
	}
}

impl From<(&str, i8, u16, u16, f32)> for AnimatedTextureConfiguration {
	fn from(t: (&str, i8, u16, u16, f32)) -> Self {
		if t.1 != 0 {
			tracing::warn!( "AnimatedTextureConfiguration for {} from (...) is a hack. .1 is ignored, please fix your template", t.0);
		}
		Self {
			template:    t.0.to_owned(),
			first_frame: t.2,
			last_frame:  t.3,
			fps:         t.4,
		}
	}
}

#[derive(Debug)]
pub struct AnimatedTexture {
	template:              String,
	first_frame:           u16,
	number_of_frames:      u16,
	fps:                   f32,
	current_frame:         u16,
	time_per_frame:        f32,
	time_in_current_frame: f32,
	autoloop:              bool,
	completed:             bool,
}

impl AnimatedTexture {
	pub fn new() -> Self {
		Self {
			template:              String::new(),
			first_frame:           0,
			number_of_frames:      0,
			fps:                   0.0,
			current_frame:         0,
			time_per_frame:        f32::MAX,
			time_in_current_frame: 0.0,
			autoloop:              true,
			completed:             false,
		}
	}

	pub fn setup(&mut self, template: &str, first_frame: u16, number_of_frames: u16, fps: f32) {
		self.template = template.to_owned();
		self.first_frame = first_frame;
		self.number_of_frames = number_of_frames;
		self.fps = fps;
		self.time_per_frame = 1.0 / fps;
		self.current_frame = first_frame;
	}

	pub fn setup_from_config(&mut self, config: &AnimatedTextureConfiguration) {
		self.template = config.template.clone();
		self.first_frame = config.first_frame;
		self.number_of_frames = config.last_frame - config.first_frame;
		self.fps = config.fps;
		self.time_per_frame = 1.0 / self.fps;
		self.current_frame = self.first_frame;
	}

	pub fn set_current_frame(&mut self, f: u16) {
		let mut f = f;
		while f < self.first_frame {
			//			todo!("Clip into range");
			f += self.number_of_frames;
		}
		while f >= self.first_frame + self.number_of_frames {
			f -= self.number_of_frames;
		}

		if f < self.first_frame || f >= self.first_frame + self.number_of_frames {
			todo!("how did we get here?");
		}

		self.current_frame = f;
		self.completed = false;
	}

	pub fn set_autoloop(&mut self, autoloop: bool) {
		self.autoloop = autoloop;
	}

	pub fn update(&mut self, time_step: f64) {
		self.time_in_current_frame += time_step as f32;
		if !self.completed {
			while self.time_in_current_frame > self.time_per_frame {
				//			self.current_frame = ( self.current_frame+1 ) % self.number_of_frames;
				self.current_frame += 1;
				if self.current_frame >= self.first_frame + self.number_of_frames {
					if self.autoloop {
						self.current_frame -= self.number_of_frames;
					} else {
						self.current_frame = self.first_frame + self.number_of_frames;
						self.set_completed(true);
					}
				}
				self.time_in_current_frame -= self.time_per_frame;
			}
		}
	}

	pub fn r#use(&self, renderer: &mut Renderer) {
		//		dbg!(&self);
		let name = AnimatedTexture::fill_template(&self.template, self.current_frame);
		renderer.use_texture(&name)
	}

	// pub for easier testing
	pub fn fill_template(template: &str, number: u16) -> Cow<str> {
		let re = Regex::new(r"(.*)%((0*\d+)*)d(.*)").unwrap();
		for caps in re.captures_iter(template) {
			//			dbg!(&caps);
			if caps.len() == 5 {
				let (prefix, format, suffix) = (&caps[1], &caps[2], &caps[4]);
				//				eprintln!("{:?} {:?} {:?}", &prefix, &format, &suffix );
				let r = match format {
					// :TODO: replace via macros?
					"" => format!("{}{}{}", &prefix, number, &suffix),
					"01" => format!("{}{:01}{}", &prefix, number, &suffix),
					"02" => format!("{}{:02}{}", &prefix, number, &suffix),
					"03" => format!("{}{:03}{}", &prefix, number, &suffix),
					"04" => format!("{}{:04}{}", &prefix, number, &suffix),
					"05" => format!("{}{:05}{}", &prefix, number, &suffix),
					"06" => format!("{}{:06}{}", &prefix, number, &suffix),
					"07" => format!("{}{:07}{}", &prefix, number, &suffix),
					"08" => format!("{}{:08}{}", &prefix, number, &suffix),
					"1" => format!("{}{:1}{}", &prefix, number, &suffix),
					"2" => format!("{}{:2}{}", &prefix, number, &suffix),
					"3" => format!("{}{:3}{}", &prefix, number, &suffix),
					"4" => format!("{}{:4}{}", &prefix, number, &suffix),
					"5" => format!("{}{:5}{}", &prefix, number, &suffix),
					"6" => format!("{}{:6}{}", &prefix, number, &suffix),
					"7" => format!("{}{:7}{}", &prefix, number, &suffix),
					"8" => format!("{}{:8}{}", &prefix, number, &suffix),
					_ => format!("{}BROKEN_TEMPLATE_%{}d_{}", &prefix, &format, &suffix),
				};
				return r.into();
				//				dbg!(&caps);
			}
		}
		template.into()
	}

	pub fn set_completed(&mut self, completed: bool) {
		self.completed = completed;
	}
	pub fn completed(&self) -> bool {
		self.completed
	}

	// :HACK: Scanning the filesystem is a bad idea, the info should come from the config
	pub fn register_all(system: &mut System, renderer: &mut Renderer, template: &str) -> usize {
		let fs = system.default_filesystem_mut();

		let mut to_load = Vec::new();
		let mut i = 0;

		loop {
			let name = AnimatedTexture::fill_template(template, i);

			// :HACK: to workaround missing "exists with .*"
			let name_ext = format!("{}.png", &name);

			if fs.exists(&name_ext) {
				to_load.push(name.to_owned());
			} else {
				println!("{} does not exist", &name_ext);
				break;
			}
			i += 1;
		}

		dbg!(&to_load);

		for name in to_load.iter() {
			renderer.register_texture(Texture::create(system, &name));
		}

		//		todo!("die");
		to_load.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn template_filenames_work() -> anyhow::Result<()> {
		assert_eq!(
			"test-0012-test",
			AnimatedTexture::fill_template("test-0012-test", 12)
		);
		// NO!		assert_eq!("test-12-12-test", AnimatedTexture::fill_template( "test-%01d-%01d-test", 12 ) );
		assert_eq!(
			"test-12-test",
			AnimatedTexture::fill_template("test-%01d-test", 12)
		);
		assert_eq!(
			"test-12-test",
			AnimatedTexture::fill_template("test-%02d-test", 12)
		);
		assert_eq!(
			"test-012-test",
			AnimatedTexture::fill_template("test-%03d-test", 12)
		);
		assert_eq!(
			"test-0012-test",
			AnimatedTexture::fill_template("test-%04d-test", 12)
		);
		assert_eq!(
			"test-00012-test",
			AnimatedTexture::fill_template("test-%05d-test", 12)
		);
		assert_eq!(
			"test-000012-test",
			AnimatedTexture::fill_template("test-%06d-test", 12)
		);
		assert_eq!(
			"test-0000012-test",
			AnimatedTexture::fill_template("test-%07d-test", 12)
		);
		assert_eq!(
			"test-00000012-test",
			AnimatedTexture::fill_template("test-%08d-test", 12)
		);
		assert_eq!(
			"test-12-test",
			AnimatedTexture::fill_template("test-%1d-test", 12)
		);
		assert_eq!(
			"test-12-test",
			AnimatedTexture::fill_template("test-%2d-test", 12)
		);
		assert_eq!(
			"test- 12-test",
			AnimatedTexture::fill_template("test-%3d-test", 12)
		);
		assert_eq!(
			"test-  12-test",
			AnimatedTexture::fill_template("test-%4d-test", 12)
		);
		assert_eq!(
			"test-   12-test",
			AnimatedTexture::fill_template("test-%5d-test", 12)
		);
		assert_eq!(
			"test-    12-test",
			AnimatedTexture::fill_template("test-%6d-test", 12)
		);
		assert_eq!(
			"test-     12-test",
			AnimatedTexture::fill_template("test-%7d-test", 12)
		);
		assert_eq!(
			"test-      12-test",
			AnimatedTexture::fill_template("test-%8d-test", 12)
		);
		assert_eq!(
			"test-12-test",
			AnimatedTexture::fill_template("test-%d-test", 12)
		);
		assert_eq!(
			"test-65535-test",
			AnimatedTexture::fill_template("test-%d-test", 65535)
		);
		Ok(())
	}
}
