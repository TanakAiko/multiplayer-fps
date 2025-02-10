use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Enemy {
    pub name: String,
    pub position: Vec3,
    pub orientation: Quat,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            name: String::new(),
            position: Vec3::ZERO,
            orientation: Quat::from_rotation_y(std::f32::consts::PI),
        }
    }
}