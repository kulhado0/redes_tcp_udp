use crate::domain::tile::tile_kind::TileKind;

pub const BLOCKED: TileKind = TileKind::Blocked('|');
pub const POWER_UP: TileKind = TileKind::Blocked('*');
pub const POINT_OBJECT: TileKind = TileKind::Blocked('.');