use bevy::prelude::*;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::client::{
    resources::world_resource::MazeResource,
    systems::{
        light_system::spawn_light,
        world::{
            collider_detect_world::collider_detect_world, load_json_world::load_maze_from_json,
            models_world::spawn_world_model, map::spawn_map
        },
    },
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeResource>()
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
            ))
            .add_systems(
                Startup,
                (load_maze_from_json, spawn_world_model, spawn_light, spawn_map).chain(),
            )
            .add_systems(Update, collider_detect_world);
    }
}
