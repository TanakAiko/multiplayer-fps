use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// const INITIAL_POSITION_PLAYER: Vec3 = Vec3::new(-12., 1.2, 13.);
// const INITIAL_ROTATION_PLAYER: Quat = Quat::IDENTITY;

use crate::client::{
    components::{
        camera_component::CameraSensitivity,
        player_component::{
            AccumulatedInput, PhysicalTranslation, Player, PreviousPhysicalTranslation, Velocity,
        },
    }, resources::player_resource::PlayerResource, systems::camera::{
        view_model_camera::spawn_view_model_camera, world_model_camera::spawn_main_camera,
    }
};

use super::view_model_player::spawn_view_model;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    camera_sensitivity: CameraSensitivity,
    accumulated_input: AccumulatedInput,
    velocity: Velocity,
    physical_translation: PhysicalTranslation,
    previous_physical_translation: PreviousPhysicalTranslation,
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
            player: Player{
                name: res_player.name.clone(),
                position: res_player.position
            },
            camera_sensitivity: CameraSensitivity::default(),
            accumulated_input: AccumulatedInput(Vec2::ZERO),
            velocity: Velocity(Vec3::ZERO),
            physical_translation: PhysicalTranslation(res_player.position),
            previous_physical_translation: PreviousPhysicalTranslation(res_player.position),
            transform: Transform::from_xyz(
                res_player.position.x,
                res_player.position.y,
                res_player.position.z,
            ),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            collider: Collider::ball(0.1),
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
