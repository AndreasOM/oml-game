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

	pub fn traces_info() -> Vec<TraceInfo> {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.traces_info()
				} else {
					Vec::default()
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn frames() -> usize {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.frames()
				} else {
					0
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn maximum_length() -> usize {
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.maximum_length()
				} else {
					0
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
	pub fn trace<T>(name: &str, value: T)
	where
		T: TelemetryEntry,
	{
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.trace::<T>(name, value)
				}
			},
			Err(e) => {
				panic!("DefaultTelemetry -> {:?}", e);
			},
		}
	}

	pub fn get<T>(name: &str) -> Vec<Option<T>>
	where
		T: TelemetryEntry,
	{
		match DEFAULT_TELEMETRY.lock() {
			Ok(ref mut dt) => {
				if let Some(dt) = &mut **dt {
					dt.get::<T>(name)
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

#[derive(Debug)]
pub struct TraceInfo {
	id:            String,
	name:          String,
	entry_type_id: std::any::TypeId,
}

impl Default for TraceInfo {
	fn default() -> Self {
		Self {
			id:            String::default(),
			name:          String::default(),
			entry_type_id: std::any::TypeId::of::<()>(),
		}
	}
}

impl TraceInfo {
	pub fn id(&self) -> &str {
		&self.id
	}
	pub fn name(&self) -> &str {
		&self.name
	}
	pub fn entry_type_id(&self) -> std::any::TypeId {
		self.entry_type_id
	}
}
#[derive(Debug, Default, Clone)]
pub enum Entry {
	#[default]
	None,
	F32(f32),
	F64(f64),
	STRING(String),
}

impl Entry {}

pub trait TelemetryEntry: Into<Entry> + From<Entry> + core::fmt::Debug {
	fn prefix() -> &'static str;
}

impl TelemetryEntry for f64 {
	fn prefix() -> &'static str {
		"F64"
	}
}

impl From<f64> for Entry {
	fn from(v: f64) -> Self {
		Entry::F64(v)
	}
}

impl From<Entry> for f64 {
	fn from(e: Entry) -> Self {
		if let Entry::F64(v) = e {
			v
		} else {
			0.0
		}
	}
}

impl TelemetryEntry for f32 {
	fn prefix() -> &'static str {
		"F32"
	}
}

impl From<f32> for Entry {
	fn from(v: f32) -> Self {
		Entry::F32(v)
	}
}

impl From<Entry> for f32 {
	fn from(e: Entry) -> Self {
		if let Entry::F32(v) = e {
			v
		} else {
			0.0
		}
	}
}

impl TelemetryEntry for String {
	fn prefix() -> &'static str {
		"STRING"
	}
}

impl From<String> for Entry {
	fn from(v: String) -> Self {
		Entry::STRING(v)
	}
}

impl From<Entry> for String {
	fn from(e: Entry) -> Self {
		if let Entry::STRING(v) = e {
			v
		} else {
			String::default()
		}
	}
}

#[derive(Debug)]
struct Trace {
	name:          String,
	entries:       VecDeque<Option<Entry>>,
	current:       Option<Entry>,
	entry_type_id: std::any::TypeId,
}

impl Default for Trace {
	fn default() -> Self {
		Self {
			name:          String::default(),
			entries:       VecDeque::default(),
			current:       None,
			entry_type_id: std::any::TypeId::of::<()>(),
		}
	}
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
	frames:         usize,
}

impl Default for Telemetry {
	fn default() -> Self {
		Self {
			frames:         0,
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
		self.frames += 1;
	}

	pub fn frames(&self) -> usize {
		self.frames
	}

	pub fn set_maximum_length(&mut self, maximum_length: usize) {
		self.maximum_length = maximum_length;
	}
	pub fn maximum_length(&self) -> usize {
		self.maximum_length
	}

	pub fn traces_info(&self) -> Vec<TraceInfo> {
		self.traces
			.iter()
			.map(|(k, t)| TraceInfo {
				id:            k.clone(),
				name:          t.name.clone(),
				entry_type_id: t.entry_type_id,
			})
			.collect()
	}

	fn prefix_for<T>() -> &'static str
	where
		T: TelemetryEntry,
	{
		T::prefix()
	}

	fn name_for<T>(name: &str) -> String
	where
		T: TelemetryEntry,
	{
		format!("{}-{}", Self::prefix_for::<T>(), name)
	}

	fn entry_from<T>(value: T) -> Entry
	where
		T: TelemetryEntry,
	{
		value.into()
	}

	pub fn trace<T>(&mut self, name: &str, value: T)
	where
		T: TelemetryEntry,
	{
		let id = Self::name_for::<T>(name);
		let trace = self.traces.entry(id).or_insert(Trace {
			name: name.to_string(),
			..Default::default()
		});

		trace.add(Self::entry_from(value));
	}

	pub fn get<T>(&mut self, name: &str) -> Vec<Option<T>>
	where
		T: TelemetryEntry + std::convert::From<Entry>,
	{
		if let Some(trace) = &self.traces.get(&Self::name_for::<T>(name)) {
			trace
				.entries()
				.iter()
				.map(|me| me.as_ref().map(|e| T::from(e.to_owned())))
				.collect::<Vec<Option<T>>>()
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
