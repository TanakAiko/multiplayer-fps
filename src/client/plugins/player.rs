use bevy::prelude::*;

use crate::client::systems::{light::{self, spawn_light}, player::setup::setup};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup, spawn_light)) // setup le player revient a setup les cameras
            ;
    }
}
