use bevy::prelude::*;

use crate::common::types::protocol::CommonPlayer;

#[derive(Debug, Resource)]
pub struct PlayerResource {
    pub name: String,
    pub position: Vec3,
}

impl PlayerResource {
    pub fn new(plyer: CommonPlayer) -> Self {
        PlayerResource {
            name: plyer.name,
            position: plyer.position,
        }
    }
}
