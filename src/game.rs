use chrono::prelude::*;

use crate::window::Window;
use crate::App;

pub struct Game {}

impl Game {
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

		window.run(move |wuc| {
			//		dbg!(&wuc);
			match app.update(wuc) {
				Ok(_) => {},
				Err(_e) => {
					return true;
				},
			}
			app.render();
			if app.is_done() {
				println!("App is done, tearing down");
				app.teardown();
				true
			} else {
				false
			}
		});

		window.teardown();

		Ok(())
	}
}
