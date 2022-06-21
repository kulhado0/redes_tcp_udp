use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum TileKind {
	Blocked,
	PointObject,
	PowerUp,
	Filled,
	Empty,
}