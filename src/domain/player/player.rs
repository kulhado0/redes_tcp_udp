use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::domain::commons::{component::Component, position::Position, serializable_uuid};

use super::player_mode::PlayerMode;

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    #[serde(with = "serializable_uuid")]
    id: Uuid,
    name: String,
    pub position: Position,
    pub mode: PlayerMode,
    pub punctuation: i32,
}

impl Component for Player {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }
}

impl Player {
    pub fn new(name: &str) -> Self {
        Player {
            id: Uuid::new_v4(),
            name: name.to_owned(),
            mode: PlayerMode::Normal,
            position: Position { x: 0, y: 0 },
            punctuation: 0,
        }
    }
}
