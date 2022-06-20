use uuid::Uuid;

use crate::domain::commons::{position::Position, component::Component};

use super::player_mode::PlayerMode;

pub struct Player {
    id: Uuid,
    name: String,
    pub position: Position,
    pub mode: PlayerMode,
    pub punctuation: i32,
}

impl Component for Player {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name<'a>(&'a self) -> &'a str {
        self.name.as_str()
    }
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            id: Uuid::new_v4(),
            name,
            mode: PlayerMode::Normal,
            position: Position { x: 0, y: 0 },
            punctuation: 0,
        }
    }
}