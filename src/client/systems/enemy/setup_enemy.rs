use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::components::enemy_component::Enemy;

const ENEMY_INITIAL_POSITION: Vec3 = Vec3::new(-12., 1.2, 13.);
const ENEMY_INITIAL_ROTATION: Quat = Quat::IDENTITY;

#[derive(Bundle, Debug, Default)]
pub struct EnemyBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity_scale: GravityScale,
    pub active_events: ActiveEvents,
    locked_axes: LockedAxes,
    collision_types: ActiveCollisionTypes,
    damping: Damping,
    friction: Friction,
    restitution: Restitution,
}

#[derive(Bundle, Debug, Default)]
pub struct EnemyBevyBundle {
    pub enemy: Enemy,
    pub transform: Transform,
}

pub fn spawn_enemy(name: String, mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle: Handle<Scene> = asset_server.load("fps_enemy.gltf#Scene0");

    let parent = commands
        .spawn((
            EnemyBundle {
                // transform: Transform::from_translation(ENEMY_INITIAL_POSITION),
                transform: Transform {
                    translation: ENEMY_INITIAL_POSITION,
                    rotation: ENEMY_INITIAL_ROTATION,
                    scale: Vec3::ONE,
                },
                global_transform: GlobalTransform::default(),
                rigid_body: RigidBody::Fixed,
                collider: Collider::capsule_y(1.8, 0.3),
                gravity_scale: GravityScale(0.),
                active_events: ActiveEvents::default(),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
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
            },
            SceneRoot(scene_handle),
            AnimationPlayer::default(),
        ))
        .id();

    commands
        .spawn(EnemyBevyBundle {
            enemy: Enemy {
                name,
                position: ENEMY_INITIAL_POSITION,
                orientation: ENEMY_INITIAL_ROTATION,
            },
            transform: Transform {
                translation: Vec3::new(3., -5., 11.),
                rotation: ENEMY_INITIAL_ROTATION,
                scale: Vec3::ONE,
            },
        })
        .set_parent(parent);
}
