use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Join {
        name: String,
    },
    PlayerUpdateSending {
        position: Vec3,
        rotation: Quat,
    },
    PlayerUpdateReceiving {
        name: String,
        position: Vec3,
        rotation: Quat,
    },
    Leave,
}
