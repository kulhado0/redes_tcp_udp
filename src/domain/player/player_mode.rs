use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum PlayerMode {
    God,
    Enemy,
    Normal
}