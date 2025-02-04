use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::components::enemy_component::Enemy;

#[derive(Bundle, Debug, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub active_events: ActiveEvents,
}

pub fn spawn_enemy(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {

    let scene_handle: Handle<Scene> = asset_server.load("fps_enemy.gltf#Scene0");

    commands.spawn((
        EnemyBundle {
            enemy: Enemy::default(),
            transform: Transform::from_xyz(22., 0.6, 22.),
            global_transform: GlobalTransform::default(),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            gravity_scale: GravityScale(0.),
            active_events: ActiveEvents::default(),
        },
        SceneRoot(scene_handle),
        AnimationPlayer::default(),
    ));
}
