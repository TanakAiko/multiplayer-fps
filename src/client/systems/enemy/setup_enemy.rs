use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::{components::enemy_component::Enemy, resources::enemy_resource::EnemyResource};

pub const PLAYER_INITIAL_ROTATION: Quat = Quat::IDENTITY;

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

    let capsule_height = 1.01; // Hauteur du corps
    let capsule_radius = 0.305; // Rayon
    let collider_offset = capsule_height / 2.0;

    commands
        .spawn((
            SceneRoot(scene_handle.clone()), // 🔹 Modèle 3D
            AnimationPlayer::default(),
            Transform {
                translation: position, // 🔹 Position réelle de l'avatar (sans modification de Y)
                rotation: PLAYER_INITIAL_ROTATION,
                scale: Vec3::ONE,
            },
            GlobalTransform::default(),
        ))
        .with_children(|parent| {
            parent.spawn((EnemyBundle {
                enemy: Enemy {
                    name,
                    position,
                    orientation: PLAYER_INITIAL_ROTATION,
                },
                transform: Transform {
                    translation: Vec3::new(0.0, collider_offset, 0.0), // 🔹 Seul le collider est déplacé
                    ..Default::default()
                },
                visibility: Visibility::default(),
                gravity_scale: GravityScale(0.0),
                locked_axes: LockedAxes::ROTATION_LOCKED,
                collision_types: ActiveCollisionTypes::DYNAMIC_STATIC,
                active_events: ActiveEvents::COLLISION_EVENTS,
                global_transform: GlobalTransform::default(),
                rigid_body: RigidBody::Fixed,
                collider: Collider::capsule_y(capsule_height, capsule_radius),
            },));
        });
}

pub fn spawn_all_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_resource: Res<EnemyResource>,
) {
    for enemy in enemy_resource.enemies.iter() {
        println!("enemy {:?}", enemy);
        spawn_enemy(
            enemy.name.clone(),
            commands.reborrow(),
            &asset_server,
            enemy.position,
        );
    }
}
