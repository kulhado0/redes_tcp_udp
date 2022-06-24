use crate::domain::player::direction::Direction;
use phf::phf_map;

pub const UP: Direction = Direction::Up(-1);
pub const DOWN: Direction = Direction::Down(1);
pub const LEFT: Direction = Direction::Left(-1);
pub const RIGHT: Direction = Direction::Right(1);

pub static KEYS_AND_DIRECTIONS: phf::Map<&'static str, Direction> = phf_map! {
    "ArrowUp" => UP,
    "ArrowDown" => DOWN,
    "ArrowLeft" => LEFT,
    "ArrowRight" => RIGHT,
};

pub static REPRESENTATIONS_AND_DIRECTIONS: phf::Map<&'static str, Direction> = phf_map! {
    "Up" => UP,
    "Down" => DOWN,
    "Left" => LEFT,
    "Right" => RIGHT,
};