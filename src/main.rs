#[macro_use]
extern crate rocket;

mod consts;
mod domain;
mod requests;
mod responses;
mod routes;

use std::sync::RwLock;

use consts::boards;
use domain::{
    board::board_creator::BoardCreator,
    player::players_manager::PlayersManager,
};

#[get("/")]
fn index() -> String {
    format!("hello")
}

#[launch]
fn rocket() -> _ {
    let tile_symbols = boards::DEFAULT_BOARD_TILE_SYMBOLS.map(|slice| slice.to_vec());

    let board = BoardCreator::create_from(&tile_symbols, "board0");
    let players_manager = PlayersManager::new(10, &board);

    let board = RwLock::new(board);
    let players_manager = RwLock::new(players_manager);

    rocket::build()
        .manage(board)
        .manage(players_manager)
        .mount("/", routes![index])
        .mount("/players", routes![
            routes::player::get_all_players,
            routes::player::move_player,
        ])
        .mount("/board", routes![
            routes::board::get_default_board
        ])
}
