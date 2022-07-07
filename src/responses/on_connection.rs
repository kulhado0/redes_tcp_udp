use serde::Serialize;

use crate::domain::{player::player::Player, board::board::Board};

#[derive(Serialize)]
pub struct ConnectionEstablished {
    board: Board,
    players: Vec<Player>,
    new_player: Player,
}

impl ConnectionEstablished {
    pub fn new(board: Board, players: Vec<Player>, new_player: Player) -> Self {
        ConnectionEstablished {
            board,
            players,
            new_player,
        }
    }
}