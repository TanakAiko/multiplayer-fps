use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::components::flag_component::Flag;

const FLAG_POSITION: Vec3 = Vec3::new(-12., -1., 10.);

#[derive(Bundle)]
struct FlagBundle {
    flag: Flag,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    collider: Collider,
    rigid_body: RigidBody,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    collision_types: ActiveCollisionTypes,
    active_events: ActiveEvents,
}

pub fn spawn_flag(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle: Handle<Scene> = asset_server.load("fps_flag.gltf#Scene0");

    commands.spawn((
        FlagBundle {
            flag: Flag {
                position: FLAG_POSITION,
            },
            transform: Transform::from_xyz(FLAG_POSITION.x, FLAG_POSITION.y, FLAG_POSITION.z),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            gravity_scale: GravityScale(0.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            rigid_body: RigidBody::Fixed,
            collider: Collider::capsule_y(1.8, 0.2),
        },
        SceneRoot(scene_handle),
        AnimationPlayer::default(),
    ));
}
