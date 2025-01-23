use bevy::prelude::*;

use crate::client::components::{camera_component::CameraSensitivity, player_component::Player};

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    
}