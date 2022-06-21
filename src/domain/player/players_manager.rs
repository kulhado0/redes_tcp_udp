use crate::consts::{directions, boards};

use super::{player::Player, direction::Direction};

pub struct PlayersManager {
    players: Vec<Player>
}

impl PlayersManager {
    pub fn new(number_of_players: u32) -> Self {
        let mut players = Vec::with_capacity(number_of_players as usize);

        for i in 0..number_of_players {
            players.push(Player::new(format!("player{i}")))
        }

        PlayersManager { players }
    }
}

impl PlayersManager {
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn move_player(&self, p: &mut Player, direction: &Direction) {
        let mut new_position = p.position;

        match direction {
            Direction::Up(value) | Direction::Down(value) => new_position.y += value,
            Direction::Left(value) | Direction::Right(value) => new_position.x += value,
            _ => ()
        }

        p.position = new_position;
    }

    pub fn move_player_on_key_pressed(&self, p: &mut Player, key: &str) {
        if let Some(direction) = directions::KEYS_AND_DIRECTIONS.get(key) {
            self.move_player(p, direction);
        }
    }
}