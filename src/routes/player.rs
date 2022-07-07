use std::{str::FromStr, sync::RwLock};

use uuid::Uuid;

use crate::{
    consts::directions,
    domain::player::{player::Player, players_manager::PlayersManager},
    requests::on_move_message::MovePlayerInfos,
};

pub fn get_all_players(manager: &RwLock<PlayersManager>) -> Vec<Player> {
    let manager = manager.read().expect("lock failed in players");

    manager.players().to_vec()
}

pub fn get_player_by_id(id: String, manager: &RwLock<PlayersManager>) -> Result<Player, String> {
    let id_conversion_result = Uuid::from_str(id.as_str());

    if let Err(_) = id_conversion_result {
        return Err(format!("Invalid id. Must be a valid uuid"));
    }

    let manager = manager
        .read()
        .expect("lock failed for manager in get player by id");

    let converted_id = id_conversion_result.unwrap();
    let option = manager.get_player_with_id(&converted_id);

    if let Some(player) = option {
        return Ok(player.clone());
    }

    Err(format!("There is no player with id = {id}"))
}

pub fn move_player(
    infos: &MovePlayerInfos,
    manager: &mut PlayersManager,
) -> Result<Player, String> {
    let direction = directions::REPRESENTATIONS_AND_DIRECTIONS.get(&infos.direction);

    if let None = direction {
        let valid_keys = directions::REPRESENTATIONS_AND_DIRECTIONS
            .keys()
            .map(|k| *k)
            .collect::<Vec<&'static str>>()
            .join(", ");

        return Err(format!("Invalid key. Valid keys: {valid_keys}"));
    }

    let player = manager.get_player_with_id(&infos.player_id);

    if let None = player {
        return Err(format!("There is no player with id = {}", &infos.player_id));
    }

    let result = manager.move_player(&infos.player_id, &direction.unwrap());

    if let Err(error) = result {
        return Err(error);
    }

    let player = manager
        .get_player_with_id(&infos.player_id)
        .unwrap()
        .clone();

    Ok(player)
}
