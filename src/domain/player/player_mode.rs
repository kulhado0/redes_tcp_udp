use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum PlayerMode {
    God,
    Enemy,
    Normal
}