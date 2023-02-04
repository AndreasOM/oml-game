#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub enum Cardinals {
	#[default]
	Top,
	Right,
	Bottom,
	Left,
}

impl From<&Cardinals> for &str {
	fn from(c: &Cardinals) -> Self {
		match c {
			Cardinals::Top => "top",
			Cardinals::Right => "right",
			Cardinals::Bottom => "bottom",
			Cardinals::Left => "left",
		}
	}
}

impl From<&Cardinals> for String {
	fn from(c: &Cardinals) -> Self {
		let s: &str = c.into();
		s.to_string()
	}
}
