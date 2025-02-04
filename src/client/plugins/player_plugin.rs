use bevy::prelude::*;

use crate::client::systems::player::{
    move_player::move_player, rotate_player::rotate_player, setup_player::setup,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup) // setup le player revient a setup les cameras
            .add_systems(Update, (rotate_player, move_player));
    }
}
