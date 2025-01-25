use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::{
    components::{camera_component::CameraSensitivity, player_component::Player},
    systems::camera::{
        view_model_camera::spawn_view_model_camera, world_model_camera::spawn_main_camera,
    },
};

use super::{move_player::{AccumulatedInput, PhysicalTranslation, PreviousPhysicalTranslation, Velocity}, view_model_player::spawn_view_model};

// Le player instancie les camera comme enfant

/*
    Le Player est une entité qui représente l'état du joueur
     (position, rotation, etc.) et sert de conteneur pour
      d'autres composants comme les caméras.
*/
// setup_player
fn spawn_player(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            AccumulatedInput(Vec2::ZERO),
            Velocity(Vec3::ZERO),
            PhysicalTranslation(Vec3::ZERO),
            PreviousPhysicalTranslation(Vec3::ZERO),
            Transform::from_xyz(0., 5., 0.),
            GlobalTransform::default(),
            Visibility::default(),
            Collider::ball(0.5),
            RigidBody::Dynamic,
        ))
        .id()
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = spawn_player(&mut commands);

    commands.entity(player).with_children(|parent| {
        spawn_main_camera(parent);
        spawn_view_model_camera(parent);
        spawn_view_model(parent, &mut meshes, &mut materials);
    });
}
