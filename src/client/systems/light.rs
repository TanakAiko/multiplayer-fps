use bevy::{color::palettes::tailwind, prelude::*, render::view::RenderLayers};

use super::camera::setup::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER};

pub fn spawn_light(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color:  Color::from(tailwind::ORANGE_200),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2., 4., 0.75),
        RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
    ));
}