#[macro_use]
extern crate rocket;

mod consts;
mod domain;
mod request_data;

use std::sync::RwLock;

use consts::boards;
use domain::{
    board::{board::Board, board_creator::BoardCreator},
    player::{player::Player, players_manager::PlayersManager},
};
use rocket::{serde::json::Json, State};

use crate::{consts::directions, request_data::move_player_infos::MovePlayerInfos};

#[get("/")]
fn index() -> String {
    format!("hello")
}

#[get("/")]
fn board(board: &State<RwLock<Board>>) -> Json<Board> {
    let board = board.read().expect("lock failed in board");

    Json(board.clone())
}

#[get("/")]
fn players(manager: &State<RwLock<PlayersManager>>) -> Json<Vec<Player>> {
    let manager = manager.read().expect("lock failed in players");

    Json(manager.players().to_vec())
}

#[put("/move", format = "json", data = "<infos>")]
fn move_player(
    infos: Json<MovePlayerInfos>,
    manager: &State<RwLock<PlayersManager>>,
) -> Result<Json<Player>, String> {
    let direction = directions::REPRESENTATIONS_AND_DIRECTIONS.get(&infos.0.direction);

    if let None = direction {
        let valid_keys = directions::REPRESENTATIONS_AND_DIRECTIONS
            .keys()
            .map(|k| *k)
            .collect::<Vec<&'static str>>()
            .join(", ");

        return Err(format!("Invalid key. Valid keys: {valid_keys}",));
    }

    let mut manager = manager.write().expect("lock failed in move_player");

    let result = manager.move_player(&infos.0.player_id, &direction.unwrap());

    if let Err(error) = result {
        return Err(error);
    }

    Ok(Json(manager.get_player_with_id(&infos.0.player_id).unwrap().clone()))
}

#[launch]
fn rocket() -> _ {
    let tile_symbols = boards::DEFAULT_BOARD_TILE_SYMBOLS.map(|slice| slice.to_vec());

    let board = BoardCreator::create_from(&tile_symbols, "board0");
    let board = RwLock::new(board);

    let players_manager = PlayersManager::new(10);
    let players_manager = RwLock::new(players_manager);

    rocket::build()
        .manage(board)
        .manage(players_manager)
        .mount("/", routes![index])
        .mount("/players", routes![players, move_player])
        .mount("/board", routes![board])
}
