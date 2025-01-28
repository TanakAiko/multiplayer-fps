use bevy::prelude::*;

use crate::client::systems::
    player::{
        move_player::{advance_physics, handle_input, interpolate_rendered_transform},
        rotate_player::rotate_player,
        setup_player::setup,
    };

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup) // setup le player revient a setup les cameras
            .add_systems(Update, rotate_player)
            // For player movement smoothness
            .add_systems(FixedUpdate, advance_physics)
            .add_systems(
                RunFixedMainLoop,
                (
                    handle_input.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
                    interpolate_rendered_transform
                        .in_set(RunFixedMainLoopSystem::AfterFixedMainLoop),
                ),
            );
    }
}
