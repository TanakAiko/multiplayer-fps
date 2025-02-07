use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::components::enemy_component::Enemy;

const ENEMY_INITIAL_POSITION: Vec3 = Vec3::new(-12., -1., 13.); // C'est en faite la meme position que le player
const ENEMY_INITIAL_ROTATION: Quat = Quat::IDENTITY;

#[derive(Bundle, Debug, Default)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
}

pub fn spawn_enemy(name: String, mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle: Handle<Scene> = asset_server.load("fps_enemy.gltf#Scene0");

    commands
        .spawn((
            EnemyBundle {
                enemy: Enemy {
                    name,
                    position: ENEMY_INITIAL_POSITION,
                    orientation: ENEMY_INITIAL_ROTATION,
                },
                // transform: Transform::from_translation(ENEMY_INITIAL_POSITION),
                transform: Transform {
                    translation: ENEMY_INITIAL_POSITION,
                    rotation: ENEMY_INITIAL_ROTATION,
                    scale: Vec3::ONE,
                },
                global_transform: GlobalTransform::default(),
                rigid_body: RigidBody::Fixed,
                collider: Collider::capsule_y(1.8, 0.3),
            },
            SceneRoot(scene_handle),
            AnimationPlayer::default(),
        ));
}
