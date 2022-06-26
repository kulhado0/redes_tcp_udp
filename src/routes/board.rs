use std::sync::RwLock;

use crate::domain::{board::board::Board, tile::tile::Tile};

pub fn get_default_board(board: &RwLock<Board>) -> Board {
    let board = board.read().expect("lock failed in board");

    board.clone()
}

pub fn get_default_board_tiles(board: &RwLock<Board>) -> Vec<Vec<Tile>> {
    let board = board.read().expect("lock failed in board");

    board.clone().tiles().to_vec()
}
