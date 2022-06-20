#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TileKind {
	Blocked(char),
	PointObject(char),
	PowerUp(char),
	Filled,
	Empty,
}

impl TileKind {
	pub fn symbol(&self) -> Option<char> {
		match *self {
			Self::Blocked(c) => Some(c),
			Self::PointObject(c) => Some(c),
			Self::PowerUp(c) => Some(c),
			_ => None,
		}
	}
}