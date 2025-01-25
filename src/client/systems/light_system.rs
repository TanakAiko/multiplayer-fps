use bevy::{ prelude::*, render::view::RenderLayers};

use super::camera::setup_camera::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            // color:  Color::from(tailwind::ORANGE_200),
            shadows_enabled: true,
            ..default()
        },
        // Transform::from_xyz(-2., 4., 0.75),
        Transform::from_xyz(0., 5., 0.),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0., 5., -1.),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}