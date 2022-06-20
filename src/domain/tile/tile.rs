use super::tile_kind::TileKind;
use crate::domain::commons::position::Position;

pub struct Tile {
    kind: TileKind,
    position: Position,
}

impl Tile {
    pub fn new(kind: TileKind, position: Position) -> Self {
        Tile { kind, position }
    }

    pub fn kind(&self) -> &TileKind {
        &self.kind
    }

    pub fn position(&self) -> Position {
        self.position
    }
}
