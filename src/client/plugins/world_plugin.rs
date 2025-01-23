use bevy::prelude::*;

use crate::client::systems::world::models_world::{self, spawn_world_model};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(Startup, spawn_world_model);
    }
}
