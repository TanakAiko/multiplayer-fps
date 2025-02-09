use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonPlayer {
    pub name: String,
    pub position: Vec3,
}

impl CommonPlayer {
    pub fn new() -> Self {
        CommonPlayer {
            name: String::new(),
            position: Vec3::ZERO,
        }
    }
}

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
    StartGame {
        player: CommonPlayer,
        enemies: Vec<CommonPlayer>,
    },
    GameOver,
    Win,
    Lose,
    Leave,
}
