//use chrono::prelude::*;
use glutin::dpi::PhysicalPosition;
use glutin::event::VirtualKeyCode;
use glutin::event::{ElementState, Event, KeyboardInput, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::{ContextWrapper, PossiblyCurrent};
use tracing::*;

use crate::math::Vector2;
pub use crate::window::window_update_context::WindowUpdateContext;

const TARGET_FPS: f64 = 60.0;
const TARGET_FRAME_TIME: f64 = 1000.0 / TARGET_FPS;

#[derive(Default)]
#[allow(dead_code)]
pub struct WindowCallbacks {
	update: Option<Box<dyn FnMut(&mut Box<dyn WindowUserData>, &mut WindowUpdateContext) -> bool>>,
	fixed_update: Option<Box<dyn FnMut(&mut Box<dyn WindowUserData>, f64)>>,
	render:       Option<Box<dyn FnMut(&mut Box<dyn WindowUserData>)>>,
}

impl<'a> WindowCallbacks {
	//	pub fn with_update( mut self, f: &'a mut (dyn for<'r> FnMut(&'r mut WindowUpdateContext) -> bool + 'a) ) -> Self {
	pub fn with_update(
		mut self,
		f: Box<dyn FnMut(&mut Box<dyn WindowUserData>, &mut WindowUpdateContext) -> bool>,
	) -> Self {
		self.update = Some(f);
		self
	}
	pub fn with_fixed_update(
		mut self,
		f: Box<dyn FnMut(&mut Box<dyn WindowUserData>, f64)>,
	) -> Self {
		self.fixed_update = Some(f);
		self
	}
	pub fn with_render(mut self, f: Box<dyn FnMut(&mut Box<dyn WindowUserData>)>) -> Self {
		self.render = Some(f);
		self
	}
}

pub trait WindowUserData {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct Window {
	el:               Option<EventLoop<()>>,
	windowed_context: Option<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
	title:            String,
	pos:              Vector2,
	size:             Vector2,
}

impl Window {
	pub fn new() -> Self {
		Self {
			el:               None,
			windowed_context: None,
			title:            String::new(),
			pos:              Vector2::new(100.0, 100.0),
			size:             Vector2::new(1400.0, 700.0),
		}
	}

	// some form of configuration
	pub fn set_title(&mut self, title: &str) {
		self.title = title.to_string();
		// if the window is already open fix the title
		if let Some(ctx) = &mut self.windowed_context {
			ctx.window().set_title(&self.title);
		}
	}

	pub fn set_position(&mut self, pos: &Vector2) {
		self.pos = *pos;
	}

	pub fn set_size(&mut self, size: &Vector2) {
		self.size = *size;
	}

	pub fn setup(&mut self) -> anyhow::Result<()> {
		let el = EventLoop::new();
		let wb = WindowBuilder::new()
			//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 1920/2, height: 1080/2 } )
			//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 1920/2, height: 512 } )
			.with_inner_size(glutin::dpi::PhysicalSize {
				width:  self.size.x as i32,
				height: self.size.y as i32,
			})
			//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 1880, height: 700 } )
			//	    			.with_inner_size( glutin::dpi::PhysicalSize{ width: 512, height: 512 } )
			.with_position(glutin::dpi::PhysicalPosition {
				x: self.pos.x as i32,
				y: self.pos.y as i32,
			})
			.with_title(&self.title);

		let windowed_context = ContextBuilder::new()
			.with_vsync(true) // yes?
			.build_windowed(wb, &el)
			.unwrap();

		let windowed_context = unsafe { windowed_context.make_current().unwrap() };

		println!(
			"Pixel format of the window's GL context: {:?}",
			windowed_context.get_pixel_format()
		);

		//	    let window = windowed_context.window();
		//	    window.set_outer_position( glutin::dpi::PhysicalPosition{ x: 2300, y: 100 } );
		self.el = Some(el);
		self.windowed_context = Some(windowed_context);

		Ok(())
	}

	pub fn teardown(&mut self) {}

	pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
		match &self.windowed_context {
			Some(windowed_context) => windowed_context.get_proc_address(addr),
			None => std::ptr::null(),
		}
	}

	fn run_event_loop(
		mut parent_thread: Option<std::thread::Thread>,
		el: EventLoop<()>,
		windowed_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
		mut userdata: Box<dyn WindowUserData>,
		mut callbacks: WindowCallbacks,
	) {
		// let el = window.el.take().unwrap();
		// let windowed_context = window.windowed_context.take().unwrap();
		let mut is_done = false;
		let mut window_update_context = WindowUpdateContext::new();

		//let mut previous_now: DateTime<Utc> = Utc::now();
		let mut previous_now = std::time::Instant::now();

		let mut event_count = 0;
		let mut next_time = std::time::Instant::now();

		let mut slowest_frame_ms = 0.0;
		let mut slow_frame_count = 0;

		el.run(move |event, _, control_flow| {
			event_count += 1;
			let start_time = std::time::Instant::now();

			window_update_context.window_size.x =
				windowed_context.window().inner_size().width as f32;
			window_update_context.window_size.y =
				windowed_context.window().inner_size().height as f32;

			match windowed_context.window().inner_position() {
				Ok(PhysicalPosition { x, y }) => {
					let x = x as f32;
					let y = y as f32;
					if window_update_context.window_pos.x != x {
						window_update_context.window_pos.x = x;
						window_update_context.window_changed = true;
					}
					if window_update_context.window_pos.y != y {
						window_update_context.window_pos.y = y;
						window_update_context.window_changed = true;
					}
				},
				_ => {},
			}
			/*
			match windowed_context.window().outer_position() {
				Ok(PhysicalPosition{ x, y }) => {
					let x = x as f32;
					let y = y as f32;
					if window_update_context.window_pos.x != x {
						window_update_context.window_pos.x = x;
						window_update_context.window_changed = true;
					}
					if window_update_context.window_pos.y != y {
						window_update_context.window_pos.y = y;
						window_update_context.window_changed = true;
					}
				},
				_ => {},
			}
			*/
			match event {
				Event::LoopDestroyed => return,
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::Resized(physical_size) => {
						dbg!(&physical_size);
						windowed_context.resize(physical_size)
					},
					WindowEvent::Moved(physical_size) => {
						dbg!(&physical_size);
						//	                	windowed_context.resize(physical_size)
					},
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
					WindowEvent::CursorMoved { position, .. } => {
						let inner_size = windowed_context.window().inner_size();

						let w = inner_size.width as f64;
						let h = inner_size.height as f64;
						let mouse_x = position.x / w;
						let mouse_y = (h - position.y) / h;
						window_update_context.mouse_pos.x = mouse_x as f32;
						window_update_context.mouse_pos.y = mouse_y as f32;
						//	                	dbg!(&position, &inner_size, &mouse_x, &mouse_y);
					},
					WindowEvent::MouseInput { state, button, .. } => {
						let button_index = match button {
							glutin::event::MouseButton::Left => 0,
							glutin::event::MouseButton::Middle => 1,
							glutin::event::MouseButton::Right => 2,
							_ => 0,
						};
						window_update_context.mouse_buttons[button_index] =
							state == glutin::event::ElementState::Pressed;

						//	                	dbg!(&state, &button, &window_update_context.mouse_buttons);
					},
					WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								virtual_keycode: Some(virtual_code),
								state,
								..
							},
						..
					} => match (virtual_code, state) {
						(VirtualKeyCode::Escape, state) => {
							window_update_context.is_escape_pressed =
								state == ElementState::Pressed;
							//                			println!("Escape {:?}", &state );
						},
						(VirtualKeyCode::Space, state) => {
							window_update_context.is_space_pressed = state == ElementState::Pressed;
							//                			println!("Space {:?}", &state );
						},
						(vkc, state) if vkc >= VirtualKeyCode::A && vkc <= VirtualKeyCode::Z => {
							let o = ((vkc as u16) - (VirtualKeyCode::A as u16)) as u8;
							let o = (o + 'a' as u8) as usize;
							println!("KeyboardInput A-Z {:?} -> {}", &vkc, &o);
							window_update_context.is_key_pressed[o] =
								state == ElementState::Pressed;
						},
						_ => {
							// println!("KeyboardInput {:?}", &virtual_code);
							if let Some(ascii) = match virtual_code {
								VirtualKeyCode::Equals => Some(61),
								VirtualKeyCode::LBracket => Some(91),
								VirtualKeyCode::Backslash => Some(92),
								VirtualKeyCode::RBracket => Some(93),
								VirtualKeyCode::Caret => Some(94),
								_ => None,
							} {
								window_update_context.is_key_pressed[ascii] =
									state == ElementState::Pressed;
							} else if let Some(fkey) = match virtual_code {
								VirtualKeyCode::F1 => Some(1),
								VirtualKeyCode::F2 => Some(2),
								VirtualKeyCode::F3 => Some(3),
								VirtualKeyCode::F4 => Some(4),
								VirtualKeyCode::F5 => Some(5),
								VirtualKeyCode::F6 => Some(6),
								VirtualKeyCode::F7 => Some(7),
								VirtualKeyCode::F8 => Some(8),
								VirtualKeyCode::F9 => Some(9),
								VirtualKeyCode::F10 => Some(10),
								VirtualKeyCode::F11 => Some(11),
								VirtualKeyCode::F12 => Some(12),
								_ => None,
							} {
								window_update_context.is_function_key_pressed[fkey] =
									state == ElementState::Pressed;
							} else {
								println!("Unmapped KeyboardInput {:?} !", &virtual_code);
							}
						},
					},
					_ => (),
				},
				Event::RedrawRequested(_) => {
					//	                gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
					windowed_context.swap_buffers().unwrap();
				},
				Event::RedrawEventsCleared => {
					// :TODO: :HACK: swapped RedrawEventsCleared and MainEventsCleared for testing
					// debug!("RedrawEventsCleared {}", event_count);

					// all evens handled, lets render
					//let now: DateTime<Utc> = Utc::now();
					let now = std::time::Instant::now();
					//let frame_duration = now.signed_duration_since(previous_now);
					let frame_duration = now - previous_now;
					//let time_step = frame_duration.num_milliseconds() as f64 / 1000.0;
					let time_step = frame_duration.as_secs_f64();
					previous_now = now;
					window_update_context.time_step = time_step;

					let done = if let Some(ref mut ucb) = callbacks.update {
						ucb(&mut userdata, &mut window_update_context)
					} else {
						true
					};

					if !is_done && done {
						println!("update returned false");
						*control_flow = ControlFlow::Exit;
						is_done = true;
						if let Some(parent_thread) = parent_thread.take() {
							parent_thread.unpark();
						}
					}

					if !is_done {
						if let Some(ref mut fucb) = callbacks.fixed_update {
							// :TODO: make time step fixed
							let half_time_step = 0.5 * time_step;
							fucb(&mut userdata, half_time_step);
							fucb(&mut userdata, half_time_step);
						}
					}

					if let Some(ref mut rcb) = callbacks.render {
						rcb(&mut userdata);
					}

					window_update_context.update();
					windowed_context.swap_buffers().unwrap();
					match *control_flow {
						glutin::event_loop::ControlFlow::Exit => {},
						_ => {
							//	        println!("{:?}", event);
							//	        *control_flow = ControlFlow::Poll;
							let elapsed_time = std::time::Instant::now()
								.duration_since(start_time)
								.as_millis() as f64;
							let wait_millis = match TARGET_FRAME_TIME >= elapsed_time {
								true => {
									//debug!("Fast frame {} > {} (ms)", elapsed_time, TARGET_FRAME_TIME);
									TARGET_FRAME_TIME - elapsed_time
								},
								false => {
									warn!(
										"Slow frame {} > {} (ms)",
										elapsed_time, TARGET_FRAME_TIME
									);
									if slowest_frame_ms < elapsed_time {
										slowest_frame_ms = elapsed_time;
									}
									slow_frame_count += 1;

									0.0
								},
							};
							//debug!("Waiting {}", wait_millis);
							//let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
							let next_frame_time = std::time::Instant::now()
								+ std::time::Duration::from_millis(wait_millis as u64);
							*control_flow =
								glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
							next_time = next_frame_time;
							// *control_flow = glutin::event_loop::ControlFlow::Wait;
							// *control_flow = glutin::event_loop::ControlFlow::Poll;
						},
					}
				},
				Event::MainEventsCleared => {
					// debug!("MainEventsCleared");
				},
				Event::NewEvents(_) => {
					event_count = 0;
					/*
					debug!("--------");
					let late = if start_time > next_time {
						(start_time - next_time).as_secs_f64()
					} else {
						-(next_time - start_time).as_secs_f64()
					};
					debug!("{:?} - {:?}", start_time, next_time);
					debug!("Late: {}", late);
					*/
				},
				Event::DeviceEvent { .. } => { // :TODO: handle Button
				},
				e => {
					println!("Unhandled event: {:?}", e);
				},
			}
			/*
				*/
		});
	}
	pub fn run(
		&mut self,
		parent_thread: Option<std::thread::Thread>,
		userdata: Box<dyn WindowUserData>,
		callbacks: WindowCallbacks,
	) {
		let el = self.el.take().unwrap();
		let windowed_context = self.windowed_context.take().unwrap();

		// glutin's EventLoop run hijacks the current thread and never returns it, so we have to work around that
		Window::run_event_loop(parent_thread, el, windowed_context, userdata, callbacks);
	}
}
