#[macro_use] extern crate rocket;

mod domain;
mod consts;

use domain::{player::{player::Player, players_manager::PlayersManager}, board::{board::Board, board_creator::BoardCreator}};
use consts::{boards};
use rocket::{serde::json::Json, State};

#[get("/")]
fn index() -> String {
    format!("hello")
}

#[get("/")]
fn players(manager: &State<PlayersManager>) -> Json<&[Player]> {
    Json(manager.players())
}

#[get("/")]
fn board(board: &State<Board>) -> Json<&Board> {
    Json(board)
}

#[launch]
fn rocket() -> _ {
    let tile_symbols = boards::DEFAULT_BOARD_TILE_SYMBOLS
        .map(|slice| slice.to_vec());

    let board = BoardCreator::create_from(&tile_symbols);

    let players_manager = PlayersManager::new(10);

    rocket::build()
        .manage(board)
        .manage(players_manager)
        .mount("/", routes![index])
        .mount("/players", routes![players])
        .mount("/board", routes![board])
}