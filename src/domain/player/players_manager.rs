use uuid::Uuid;

use crate::domain::{board::board::Board, commons::component::Component};

use super::{direction::Direction, player::Player};

pub struct PlayersManager {
    players: Vec<Player>,
    board: Board,
}

impl PlayersManager {
    pub fn new(board: &Board) -> Self {
        PlayersManager {
            players: Vec::new(),
            board: board.clone(),
        }
    }
}

impl PlayersManager {
    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn get_player_with_id(&self, id: &Uuid) -> Option<&Player> {
        self.players.iter().find(|p| p.id().eq(id))
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn move_player(&mut self, player_id: &Uuid, direction: &Direction) -> Result<(), String> {
        let player = self.players.iter_mut().find(|p| p.id().eq(player_id));

        if let None = player {
            return Err(format!("There is no player with id = {}", player_id));
        }

        let player = player.unwrap();

        let mut new_position = player.position.clone();

        match direction {
            Direction::Up(value) | Direction::Down(value) => new_position.y += value,
            Direction::Left(value) | Direction::Right(value) => new_position.x += value,
            _ => (),
        }

        if self.board.is_inside(new_position) {
            player.position = new_position;
            return Ok(());
        }

        Err(format!("Position {new_position} is not inside board"))
    }
}
