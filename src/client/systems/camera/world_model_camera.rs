use bevy::prelude::*;

use crate::client::components::camera_component::WorldModelCamera;

// Ici on instancie la camera liee au monde 
pub fn spawn_main_camera(parent: &mut ChildBuilder) {
    parent.spawn((
        WorldModelCamera,
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        })
    )
    );
}
