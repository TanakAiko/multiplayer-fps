use bevy::{pbr::NotShadowCaster, prelude::*, render::view::RenderLayers};
use bevy_rapier3d::prelude::*;

use crate::client::systems::camera::setup_camera::VIEW_MODEL_RENDER_LAYER;

// C'est la qu'on dessine le player
pub fn spawn_view_model(parent: &mut ChildBuilder, asset_server: Res<AssetServer>) {
    let scene_handle: Handle<Scene> = asset_server.load("fps_player.gltf#Scene0");

    parent.spawn((
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER), // Rendu par la caméra du modèle de vue
        Transform {
            translation: Vec3::new(0.0, -0.4, -0.4),
            rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            scale: Vec3::new(0.05, 0.05, 0.05),
        },
        SceneRoot(scene_handle),
        Collider::cuboid(0.2, 0.2, 0.2),
        RigidBody::Dynamic,
        GravityScale(0.),
        LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED, // Bloque tous les axes
        NotShadowCaster::default(),
    ));
}
