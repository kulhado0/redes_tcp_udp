use phf::{phf_map};

use crate::domain::tile::tile_kind::TileKind;

pub static SYMBOLS_AND_KINDS: phf::Map<char, TileKind> = phf_map! {
    '|' => TileKind::Blocked,
    '*' => TileKind::PowerUp,
    '.' => TileKind::PointObject,
    ' ' => TileKind::Empty,
};