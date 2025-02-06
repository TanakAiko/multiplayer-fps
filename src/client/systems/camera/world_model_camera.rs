use bevy::{prelude::*, render::view::RenderLayers};
use crate::client::components::camera_component::WorldModelCamera;

use super::setup_camera::DEFAULT_RENDER_LAYER;

// Ici on instancie la camera liee au monde 
pub fn spawn_main_camera(parent: &mut ChildBuilder) {
    parent.spawn((
        WorldModelCamera,
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::layer(DEFAULT_RENDER_LAYER),
        // Collider::ball(0.5),
        // RigidBody::Dynamic,
    )
    );
}
