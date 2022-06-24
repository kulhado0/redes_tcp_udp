use std::sync::RwLock;

use rocket::{serde::json::Json, State};

use crate::domain::{board::board::Board, tile::tile::Tile};

#[get("/")]
pub fn get_default_board(board: &State<RwLock<Board>>) -> Json<Board> {
    let board = board.read().expect("lock failed in board");

    Json(board.clone())
}

#[get("/tiles")]
pub fn get_default_board_tiles(board: &State<RwLock<Board>>) -> Json<Vec<Vec<Tile>>> {
    let board = board.read().expect("lock failed in board");

    Json(board.clone().tiles().to_vec())
}
