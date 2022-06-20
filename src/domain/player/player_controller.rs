use crate::consts::directions;

use super::{player::Player, direction::Direction};

pub struct PlayerController;

impl PlayerController {
    pub fn move_player(p: &mut Player, direction: &Direction) {
        let mut new_position = p.position;

        match direction {
            Direction::Up(value) | Direction::Down(value) => new_position.y += value,
            Direction::Left(value) | Direction::Right(value) => new_position.x += value,
            _ => ()
        }

        p.position = new_position;
    }

    pub fn move_player_when_key_pressed(p: &mut Player, key: &str) {
        if let Some(direction) = directions::KEYS_AND_DIRECTIONS.get(key) {
            Self::move_player(p, direction);
        }
    }
}