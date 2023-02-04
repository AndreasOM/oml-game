// Warning: Do NOT toggle in multi threaded environment
// set up once, and leave be
// :TODO:

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static DEFAULT_TELEMETRY: Lazy<Arc<Mutex<Option<Telemetry>>>> =
	Lazy::new(|| Arc::new(Mutex::new(None)));

#[derive(Debug, Default)]
pub struct DefaultTelemetry {}

impl DefaultTelemetry {
	pub fn enable() {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if dt.is_none() {
					**dt = Some(Telemetry::default());
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn disable() {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if dt.is_some() {
					**dt = None;
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn toggle() {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if dt.is_none() {
					**dt = Some(Telemetry::default());
				} else {
					**dt = None;
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn is_enabled() -> bool {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => dt.is_some(),
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn update() {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.update()
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn set_maximum_length(maximum_length: usize) {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.set_maximum_length(maximum_length)
				} else {
					panic!("DefaultTelemetry tried to set_maximum_length while disabled");
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}
	pub fn trace_f32(name: &str, value: f32) {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.trace_f32(name, value)
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn get_f32(name: &str) -> Vec<Option<f32>> {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.get_f32(name)
				} else {
					Vec::new()
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}
}

#[derive(Debug, Default)]
enum Entry {
	#[default]
	None,
	F32(f32),
}

#[derive(Debug, Default)]
struct Trace {
	entries: VecDeque<Option<Entry>>,
	current: Option<Entry>,
}

impl Trace {
	pub fn add(&mut self, entry: Entry) {
		//self.entries.push_back( entry );
		self.current = Some(entry);
	}

	pub fn update(&mut self) {
		self.entries.push_back(self.current.take());
	}
	pub fn enforce_maximum(&mut self, maximum: usize) {
		while self.entries.len() > maximum {
			self.entries.pop_front();
		}
	}

	pub fn entries(&self) -> &VecDeque<Option<Entry>> {
		&self.entries
	}
}

#[derive(Debug)]
pub struct Telemetry {
	maximum_length: usize,
	traces:         HashMap<String, Trace>,
}

impl Default for Telemetry {
	fn default() -> Self {
		Self {
			maximum_length: 1000,
			traces:         HashMap::new(),
		}
	}
}

impl Telemetry {
	pub fn update(&mut self) {
		for (_, trace) in self.traces.iter_mut() {
			trace.update();
			trace.enforce_maximum(self.maximum_length);
		}
	}

	pub fn set_maximum_length(&mut self, maximum_length: usize) {
		self.maximum_length = maximum_length;
	}

	pub fn trace_f32(&mut self, name: &str, value: f32) {
		let trace = self.traces.entry(format!("F32-{}", name)).or_default();

		trace.add(Entry::F32(value));
	}

	pub fn get_f32(&mut self, name: &str) -> Vec<Option<f32>> {
		if let Some(trace) = &self.traces.get(&format!("F32-{}", name)) {
			trace
				.entries()
				.iter()
				.map(|me| {
					me.as_ref().map({
						|e| {
							if let Entry::F32(value) = e {
								*value
							} else {
								//todo!("Should never happen");
								0.0
							}
						}
					})
				})
				.collect::<Vec<Option<f32>>>()
		} else {
			Vec::new()
		}
	}
}

#[cfg(test)]
mod tests {
	//use super::*;
	use crate::DefaultTelemetry;
	#[test]
	fn can_toggle_default() {
		DefaultTelemetry::disable();
		assert_eq!(false, DefaultTelemetry::is_enabled());
		DefaultTelemetry::toggle();
		assert_eq!(true, DefaultTelemetry::is_enabled());
		DefaultTelemetry::toggle();
		assert_eq!(false, DefaultTelemetry::is_enabled());
	}

	#[test]
	fn can_trace_f32() {
		DefaultTelemetry::enable();
		assert_eq!(true, DefaultTelemetry::is_enabled());
		DefaultTelemetry::trace_f32("test_f32", 0.0);
		DefaultTelemetry::update();
		DefaultTelemetry::trace_f32("test_f32", 1.0);
		DefaultTelemetry::update();
		DefaultTelemetry::trace_f32("test_f32", 2.0);
		DefaultTelemetry::update();
		let t = DefaultTelemetry::get_f32("test_f32");
		assert_eq!(3, t.len());
		assert_eq!([0.0, 1.0, 2.0].to_vec(), t);

		DefaultTelemetry::set_maximum_length(10);
		for _ in 0..200 {
			DefaultTelemetry::trace_f32("test_f32", 99.0);
		}
		DefaultTelemetry::update();
		let t = DefaultTelemetry::get_f32("test_f32");
		assert_eq!(10, t.len());
	}
}
