use std::sync::RwLock;

use rocket::{serde::json::Json, State};

use crate::domain::board::board::Board;

#[get("/")]
pub fn get_default_board(board: &State<RwLock<Board>>) -> Json<Board> {
    let board = board.read().expect("lock failed in board");

    Json(board.clone())
}
