use bevy::prelude::*;

use crate::client::{components::{camera_component::CameraSensitivity, player_component::Player}, systems::camera::{view_model_camera::spawn_view_model_camera, world_model_camera::spawn_main_camera}};

use super::view_model_player::spawn_view_model;

// Le player instancie les camera comme enfant

/* 
    Le Player est une entité qui représente l'état du joueur
     (position, rotation, etc.) et sert de conteneur pour
      d'autres composants comme les caméras.
*/

fn spawn_player(commands: &mut Commands) -> Entity {
    commands.spawn((
        Player,
        CameraSensitivity::default(),
        Transform::from_xyz(0., 1., 0.),
        Visibility::default(),
    ))
    .id()
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let player = spawn_player(&mut commands);

    commands.entity(player).with_children(
        |parent| {
            spawn_main_camera(parent);
            spawn_view_model_camera(parent);
            spawn_view_model(parent, &mut meshes, &mut materials);
        }
    );
}