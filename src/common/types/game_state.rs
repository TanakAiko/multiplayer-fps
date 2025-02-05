use bevy::math::{Quat, Vec3};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug)]
pub enum GameMessage {
    PlayerUpdate {
        position: Vec3,
        rotation: Quat,
        velocity: Vec3,
        timestamp: u64,
    },
}