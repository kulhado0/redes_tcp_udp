use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::domain::commons::serializable_uuid;

#[derive(Serialize, Deserialize)]
pub struct MovePlayerInfos {
    #[serde(with = "serializable_uuid")]
    pub player_id: Uuid,
    pub direction: String,
}