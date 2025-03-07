use bevy::prelude::*;

use crate::client::resources::enemy_resource::EnemyState;

#[derive(Debug, Component)]
pub struct Enemy {
    pub name: String,
    pub position: Vec3,
    pub orientation: Quat,
    pub current_state: EnemyState,
}



impl Default for Enemy {
    fn default() -> Self {
        Self {
            name: String::new(),
            position: Vec3::ZERO,
            orientation: Quat::from_rotation_y(std::f32::consts::PI),
            current_state: EnemyState::Idle,
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct EnemyMovement {
    pub target_position: Vec3,
    pub current_position: Vec3,
    pub lerp_time: f32,
}