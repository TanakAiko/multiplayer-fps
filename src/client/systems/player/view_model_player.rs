use bevy::{
    color::palettes::tailwind, pbr::NotShadowCaster, prelude::*, render::view::RenderLayers,
};
use bevy_rapier3d::prelude::*;

use crate::client::systems::camera::setup_camera::VIEW_MODEL_RENDER_LAYER;

// C'est la qu'on dessine le player
pub fn spawn_view_model(
    parent: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let arm = meshes.add(Cuboid::new(0.01, 0.01, 0.3));
    let arm_material = materials.add(Color::from(tailwind::TEAL_400));

    parent.spawn((
        RenderLayers::layer(VIEW_MODEL_RENDER_LAYER), // Rendu par la caméra du modèle de vue
        Mesh3d(arm),
        MeshMaterial3d(arm_material),
        Transform::from_xyz(0.2, -0.1, -0.25),
        Collider::cuboid(0.01, 0.01, 0.3),
        RigidBody::Fixed,
        ActiveEvents::COLLISION_EVENTS,
        NotShadowCaster,
    ));
}
