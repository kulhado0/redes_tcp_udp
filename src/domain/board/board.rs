use crate::domain::{
    commons::{position::Position, component::Component, serializable_uuid},
    tile::{tile::Tile, tile_kind::TileKind},
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Board {
    #[serde(with = "serializable_uuid")]
    id: Uuid,
    name: String,
    tiles: Vec<Vec<Tile>>,
}

impl Component for Board {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }
}

impl Board {
    pub fn new(name: String, tiles: Vec<Vec<Tile>>) -> Self {
        Board {
            id: Uuid::new_v4(),
            name,
            tiles,
        }
    }
}

impl Board {
    pub fn tiles(&self) -> &[Vec<Tile>] {
        &self.tiles
    }

    pub fn x_limit(&self) -> usize {
        self.tiles.len()
    }

    pub fn y_limit(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn tile_at(&self, p: Position) -> Option<&Tile> {
        if let Some(item) = self.tiles.get(p.x as usize) {
            item.get(p.y as usize)
        } else {
            None
        }
    }

    pub fn is_inside_horizontaly(&self, p: Position) -> bool {
        p.x < self.x_limit() as i32
    }

    pub fn is_inside_vertically(&self, p: Position) -> bool {
        p.y < self.y_limit() as i32
    }

    pub fn is_inside(&self, p: Position) -> bool {
        self.is_inside_horizontaly(p) && self.is_inside_vertically(p)
    }

    pub fn has_tile_kind_at(&self, kind: TileKind, p: Position) -> bool {
        match self.tile_at(p) {
            Some(tile) => tile.kind().eq(&kind),
            None => false,
        }
    }
}
