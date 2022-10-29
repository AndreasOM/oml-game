use std::path::Path;

use chrono::prelude::*;
use tracing::*;

use crate::window::{Window, WindowCallbacks, WindowLayout, WindowUserData};
use crate::App;

pub struct Game {
	app: Box<dyn App>,
}

impl WindowUserData for Game {
	fn as_any(&self) -> &dyn std::any::Any {
		self
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}

impl Game {
	fn new(app: Box<dyn App>) -> Self {
		Self { app }
	}

	fn app_mut(&mut self) -> &mut Box<dyn App> {
		&mut self.app
	}
	fn app(&self) -> &Box<dyn App> {
		&self.app
	}

	pub fn run(mut app: impl App + 'static) -> anyhow::Result<()> {
		debug!("oml-game::Game::run()");

		let mut window = Window::new();

		if app.remember_window_layout() {
			let filename = app.layout_filename();
			let mut layout = WindowLayout::default();
			match layout.load(&Path::new(&filename)) {
				Ok(_) => {
					if let Some(main_layout) = layout.get_window("main") {
						println!("{:#?}", &main_layout);
						window.set_position(main_layout.pos());
						window.set_size(main_layout.size());
					}
				},
				// ignore errors
				_ => {},
			}
		}

		window.setup()?;

		let start_time: DateTime<Utc> = Utc::now();
		app.setup(&mut window)?;
		let end_time: DateTime<Utc> = Utc::now();
		let load_duration = end_time.signed_duration_since(start_time);
		let load_time = load_duration.num_milliseconds() as f64 / 1000.0;
		info!("App setup took {} seconds", load_time);

		let callbacks = WindowCallbacks::default()
			.with_update(Box::new(|wud, wuc| {
				//debug!("Update");
				match wud.as_any_mut().downcast_mut::<Game>() {
					Some(game) => {
						if wuc.window_changed && game.app().remember_window_layout() {
							let mut layout = WindowLayout::default();
							layout.set_window("main", &wuc.window_pos, &wuc.window_size);
							let filename = game.app().layout_filename();
							match layout.save(&Path::new(&filename)) {
								// :TODO: handle errors
								_ => {},
							}
						}
						match game.app_mut().update(wuc) {
							Ok(_) => {},
							Err(_e) => {
								return true;
							},
						}

						if game.app().is_done() {
							println!("App is done, tearing down");
							game.app_mut().teardown();
							return true;
						}
					},
					None => {},
				}
				false
			}))
			.with_fixed_update(Box::new(|wud, time_step| {
				//debug!("Fixed Update {}", time_step);
				match wud.as_any_mut().downcast_mut::<Game>() {
					Some(game) => {
						game.app_mut().fixed_update(time_step);
					},
					None => {},
				}
			}))
			.with_render(Box::new(|wud| {
				//debug!("Render");
				match wud.as_any_mut().downcast_mut::<Game>() {
					Some(game) => {
						game.app_mut().render();
					},
					None => {},
				}
			}));

		let game = Box::new(Game::new(Box::new(app)));

		window.run(None, game, callbacks); // never returns since thread is hijacked for good.

		window.teardown();

		Ok(())
	}
}
