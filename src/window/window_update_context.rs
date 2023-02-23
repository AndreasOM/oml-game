use crate::math::Vector2;

#[derive(Debug)]
#[repr(u8)]
pub enum ModifierKey {
	Shift,
	Ctrl,
	Alt,
}
#[derive(Debug, Copy, Clone)]
pub struct WindowUpdateContext {
	pub time_step:              f64,
	pub is_escape_pressed:      bool,
	pub is_space_pressed:       bool,
	pub mouse_pos:              Vector2,
	pub mouse_wheel_line_delta: Vector2,
	pub mouse_buttons:          [bool; 3], // left middle right
	pub is_key_pressed:         [bool; 256],
	is_modifier_pressed:        [bool; 256],
	pub window_size:            Vector2,
	pub window_pos:             Vector2,
	pub window_changed:         bool,

	previous_mouse_buttons: [bool; 3],
	previous_keys_pressed:  [bool; 256],

	pub is_function_key_pressed:    [bool; 16],
	previous_function_keys_pressed: [bool; 16],
}

impl WindowUpdateContext {
	pub fn new() -> Self {
		Self {
			time_step:               0.0,
			is_escape_pressed:       false,
			is_space_pressed:        false,
			mouse_pos:               Vector2::zero(),
			mouse_wheel_line_delta:  Vector2::zero(),
			mouse_buttons:           [false, false, false],
			is_key_pressed:          [false; 256],
			is_function_key_pressed: [false; 16],
			is_modifier_pressed:     [false; 256],
			window_size:             Vector2::zero(),
			window_pos:              Vector2::zero(),
			window_changed:          false,

			previous_mouse_buttons:         [false, false, false],
			previous_keys_pressed:          [false; 256],
			previous_function_keys_pressed: [false; 16],
		}
	}

	pub fn update(&mut self) {
		//		dbg!(&self);
		self.previous_mouse_buttons = self.mouse_buttons;
		self.previous_keys_pressed = self.is_key_pressed;
		self.previous_function_keys_pressed = self.is_function_key_pressed;
		//		for i in 0..self.is_key_pressed.len() {
		//			self.previous_keys_pressed[ i ] = self.is_key_pressed[ i ];
		//		}
	}

	pub fn fake_mouse_button_press(&mut self, button_index: usize) {
		self.mouse_buttons[button_index] = true;
		self.previous_mouse_buttons[button_index] = false;
	}

	pub fn was_mouse_button_pressed(&self, button_index: usize) -> bool {
		self.mouse_buttons[button_index] && !self.previous_mouse_buttons[button_index]
	}
	pub fn was_mouse_button_released(&self, button_index: usize) -> bool {
		!self.mouse_buttons[button_index] && self.previous_mouse_buttons[button_index]
	}

	pub fn was_key_pressed(&self, key: u8) -> bool {
		self.is_key_pressed[key as usize] && !self.previous_keys_pressed[key as usize]
	}

	pub fn is_key_pressed(&self, key: u8) -> bool {
		self.is_key_pressed[key as usize]
	}

	pub fn set_modifier_pressed(&mut self, modifier: ModifierKey, pressed: bool) {
		self.is_modifier_pressed[modifier as usize] = pressed;
	}

	pub fn is_modifier_pressed(&self, modifier: ModifierKey) -> bool {
		self.is_modifier_pressed[modifier as usize]
	}

	pub fn was_function_key_pressed(&self, key: u8) -> bool {
		self.is_function_key_pressed[key as usize]
			&& !self.previous_function_keys_pressed[key as usize]
	}

	pub fn is_function_key_pressed(&self, key: u8) -> bool {
		self.is_function_key_pressed[key as usize]
	}

	pub fn time_step(&self) -> f64 {
		self.time_step
	}
}
