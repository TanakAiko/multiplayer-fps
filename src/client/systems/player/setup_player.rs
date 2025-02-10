use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::{
    components::{camera_component::CameraSensitivity, player_component::Player},
    resources::player_resource::PlayerResource,
    systems::camera::{
        view_model_camera::spawn_view_model_camera, world_model_camera::spawn_main_camera,
    },
};

use super::view_model_player::spawn_view_model;

const PLAYER_INITIAL_ROTATION: Quat = Quat::IDENTITY;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    camera_sensitivity: CameraSensitivity,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    collider: Collider,
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
    damping: Damping,
    friction: Friction,
    restitution: Restitution,
}

// Le player instancie les camera comme enfant

/*
    Le Player est une entité qui représente l'état du joueur
     (position, rotation, etc.) et sert de conteneur pour
      d'autres composants comme les caméras.
*/
// setup_player
fn spawn_player(commands: &mut Commands, res_player: Res<PlayerResource>) -> Entity {
    commands
        .spawn(PlayerBundle {
            player: Player {
                name: res_player.name.clone(),
                position: res_player.position,
                // health: 100.
            },
            camera_sensitivity: CameraSensitivity::default(),
            transform: Transform {
                translation: res_player.position,
                rotation: PLAYER_INITIAL_ROTATION,
                scale: Vec3::ONE,
            },
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            collider: Collider::ball(0.5),
            rigid_body: RigidBody::Dynamic,
            gravity_scale: GravityScale(0.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            damping: Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            restitution: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        })
        .id()
}

pub fn setup(
    asset_server: Res<AssetServer>,
    res_player: Res<PlayerResource>,
    mut commands: Commands,
) {
    let player = spawn_player(&mut commands, res_player);

    commands.entity(player).with_children(|parent| {
        spawn_main_camera(parent);
        spawn_view_model_camera(parent);
        spawn_view_model(parent, asset_server);
    });
}
