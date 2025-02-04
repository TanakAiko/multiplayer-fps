use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Enemy {
    pub position: Vec3,
    pub orientation: Quat,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            orientation: Quat::IDENTITY,
        }
    }
}