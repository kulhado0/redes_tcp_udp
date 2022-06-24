use std::sync::RwLock;

use rocket::{http::Status, serde::json::Json, State};

use crate::{
    consts::directions,
    domain::player::{player::Player, players_manager::PlayersManager},
    requests::move_player_infos::MovePlayerInfos,
    responses::error::Error,
};

#[get("/")]
pub fn get_all_players(manager: &State<RwLock<PlayersManager>>) -> Json<Vec<Player>> {
    let manager = manager.read().expect("lock failed in players");

    Json(manager.players().to_vec())
}

#[put("/move", format = "json", data = "<infos>")]
pub fn move_player(
    infos: Json<MovePlayerInfos>,
    manager: &State<RwLock<PlayersManager>>,
) -> Result<Json<Player>, Json<Error<String>>> {
    let direction = directions::REPRESENTATIONS_AND_DIRECTIONS.get(&infos.0.direction);

    if let None = direction {
        let valid_keys = directions::REPRESENTATIONS_AND_DIRECTIONS
            .keys()
            .map(|k| *k)
            .collect::<Vec<&'static str>>()
            .join(", ");

        return Err(Json(Error::new_with_content(
            Status::BadRequest,
            format!("Invalid key. Valid keys: {valid_keys}"),
        )));
    }

    let mut manager = manager.write().expect("lock failed in move_player");

    let result = manager.move_player(&infos.0.player_id, &direction.unwrap());

    if let Err(error) = result {
        return Err(Json(Error::new_with_content(
            Status::InternalServerError,
            error,
        )));
    }

    Ok(Json(
        manager
            .get_player_with_id(&infos.0.player_id)
            .unwrap()
            .clone(),
    ))
}
