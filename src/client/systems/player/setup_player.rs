use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const INITIAL_POSITION_PLAYER: Vec3 = Vec3::new(20., 0.6, 20.);

use crate::client::{
    components::{
        camera_component::CameraSensitivity,
        player_component::{
            AccumulatedInput, PhysicalTranslation, Player, PreviousPhysicalTranslation, Velocity,
        },
    },
    systems::camera::{
        view_model_camera::spawn_view_model_camera, world_model_camera::spawn_main_camera,
    },
};

use super::view_model_player::spawn_view_model;

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
            PhysicalTranslation(INITIAL_POSITION_PLAYER),
            PreviousPhysicalTranslation(INITIAL_POSITION_PLAYER),
            Transform::from_xyz(
                INITIAL_POSITION_PLAYER.x,
                INITIAL_POSITION_PLAYER.y,
                INITIAL_POSITION_PLAYER.z,
            ),
            GlobalTransform::default(),
            Visibility::default(),
            Collider::ball(0.1),
            // // Collider::cuboid(0.5, 1., 0.5),
            ActiveEvents::COLLISION_EVENTS,
            RigidBody::Dynamic,
            GravityScale(0.),
            // ExternalForce::default(),
            // ExternalImpulse::default(),
            // LockedAxes::TRANSLATION_LOCKED
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
