use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::{components::enemy_component::Enemy, resources::enemy_resource::EnemyResource};

// const ENEMY_INITIAL_POSITION: Vec3 = Vec3::new(-12., -1., 13.); // C'est en faite la meme position que le player
const ENEMY_INITIAL_ROTATION: Quat = Quat::IDENTITY;

#[derive(Bundle, Debug, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
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

pub fn spawn_enemy(
    name: String,
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let scene_handle: Handle<Scene> = asset_server.load("fps_enemy.gltf#Scene0");

    commands.spawn((
        EnemyBundle {
            enemy: Enemy {
                name,
                position,
                orientation: ENEMY_INITIAL_ROTATION,
            },
            // transform: Transform::from_translation(ENEMY_INITIAL_POSITION),
            transform: Transform {
                translation: position,
                rotation: ENEMY_INITIAL_ROTATION,
                scale: Vec3::ONE,
            },
            visibility: Visibility::default(),
            gravity_scale: GravityScale(0.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
            active_events: ActiveEvents::COLLISION_EVENTS,
            global_transform: GlobalTransform::default(),
            rigid_body: RigidBody::Fixed,
            collider: Collider::capsule_y(1.8, 0.3),
        },
        SceneRoot(scene_handle),
        AnimationPlayer::default(),
    ));
}

pub fn spawn_all_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies: Res<EnemyResource>,
) {
    for enemy in enemies.enemies.iter() {
        println!("enemy {:?}", enemy);
        spawn_enemy(
            enemy.name.clone(),
            commands.reborrow(),
            &asset_server,
            enemy.position,
        );
    }
}
