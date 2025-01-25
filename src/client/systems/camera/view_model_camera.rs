use bevy::{prelude::*, render::view::RenderLayers};
use bevy_rapier3d::prelude::*;

use super::setup_camera::VIEW_MODEL_RENDER_LAYER;

// Cette caméra est utilisée pour afficher les objets liés au joueur 

pub fn spawn_view_model_camera(parent: &mut ChildBuilder) {
    parent.spawn((
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: 70.0_f32.to_radians(),
            ..default()
        }),
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        Collider::ball(0.5),
        RigidBody::Dynamic,
    ));
    
}

