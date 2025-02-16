use bevy::{prelude::*, text::FontSmoothing};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use crate::client::{
    resources::world_resource::MazeResource,
    systems::{
        common::light_system::spawn_light,
        world::{
            // collider_detect_world::collider_detect_world,
            load_json_world::load_maze_from_json,
            map::spawn_map,
            models_world::spawn_world_model,
        },
    },
};

pub struct WorldPlugin;
struct OverlayColor;

impl OverlayColor {
    // const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MazeResource>()
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::default(),
                RapierDebugRenderPlugin::default(),
                FpsOverlayPlugin {
                    config: FpsOverlayConfig {
                        text_config: TextFont {
                            font_size: 22.0,
                            font: default(),
                            font_smoothing: FontSmoothing::default(),
                            ..default()
                        },
                        text_color: OverlayColor::GREEN,
                        enabled: true,
                    },
                },
            ))
            .add_systems(
                Startup,
                (
                    load_maze_from_json,
                    spawn_world_model,
                    spawn_light,
                    spawn_map,
                )
                    .chain(),
            );
        // .add_systems(Update, collider_detect_world);
    }
}
