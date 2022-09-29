use chrono::prelude::*;

use crate::window::{Window, WindowCallbacks, WindowUserData};
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

	fn app(&mut self) -> &mut Box<dyn App> {
		&mut self.app
	}

	pub fn run(mut app: impl App + 'static) -> anyhow::Result<()> {
		println!("oml-game::Game::run()");

		let mut window = Window::new();

		window.setup()?;

		let start_time: DateTime<Utc> = Utc::now();
		app.setup(&mut window)?;
		let end_time: DateTime<Utc> = Utc::now();
		let load_duration = end_time.signed_duration_since(start_time);
		let load_time = load_duration.num_milliseconds() as f64 / 1000.0;
		println!("App setup took {} seconds", load_time);

		let callbacks = WindowCallbacks::default()
			.with_update(Box::new(|wud, wuc| {
				match wud.as_any_mut().downcast_mut::<Game>() {
					Some(game) => {
						match game.app().update(wuc) {
							Ok(_) => {},
							Err(_e) => {
								return true;
							},
						}

						if game.app().is_done() {
							println!("App is done, tearing down");
							game.app().teardown();
							return true;
						}
					},
					None => {},
				}
				false
			}))
			.with_render(Box::new(|wud| {
				match wud.as_any_mut().downcast_mut::<Game>() {
					Some(game) => {
						game.app().render();
					},
					None => {},
				}
			}));

		let game = Box::new(Game::new(Box::new(app)));

		window.run(game, callbacks);

		window.teardown();

		Ok(())
	}
}
