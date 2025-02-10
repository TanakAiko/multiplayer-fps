use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::client::components::{
    bullet::{Bullet, BulletDirection},
    camera_component::CameraSensitivity,
    enemy_component::Enemy,
    player_component::Player,
};

pub fn player_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    camera_query: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    if let (Ok(entity), Ok(camera_transform)) =
        (player_query.get_single(), camera_query.get_single())
    {
        if keyboard.just_pressed(KeyCode::Space) {
            let spawn_position =
                camera_transform.0.translation + camera_transform.0.forward() * 1.0;
            spawn_bullet(
                &mut commands,
                &mut meshes,
                &mut materials,
                camera_query,
                spawn_position,
                entity,
            );
        }
    }
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            speed: 20.0,
            damage: 10.0,
            lifetime: Timer::from_seconds(2.0, TimerMode::Once),
            shooter_id: Entity::from_raw(0),
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    camera_query: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
    start_position: Vec3,
    shooter_id: Entity,
) {
    if let Ok(camera_transform) = camera_query.get_single() {
        let bullet_direction: Vec3 = camera_transform.0.forward().into();
        commands.spawn((
            Mesh3d(meshes.add(Mesh::from(Sphere::new(0.1)))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Srgba::hex("#ffd891").unwrap().into(),
                ..default()
            })),
            Transform {
                translation: start_position,
                rotation: camera_transform.0.rotation,
                scale: Vec3::ONE,
            },
            BulletDirection(bullet_direction),
            Bullet {
                shooter_id,
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(0.1),
        ));
    }
}

pub fn update_bullets(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Transform, &mut Bullet, &BulletDirection)>,
) {
    for (entity, mut transform, mut bullet, direction) in bullets.iter_mut() {
        transform.translation += direction.0 * bullet.speed * time.delta_secs();
        // println!("transform: {:?}", transform);
        bullet.lifetime.tick(time.delta());
        if bullet.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &Bullet)>,
    players: Query<(Entity, &Parent), With<Enemy>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            // Try to get the bullet from entity1 first, if that fails, try entity2
            let bullet_result = bullets.get(*entity1).or_else(|_| bullets.get(*entity2));
            let other_entity = if bullets.get(*entity1).is_ok() {
                *entity2
            } else {
                *entity1
            };

            if let Ok((bullet_entity, bullet)) = bullet_result {
                if let Ok(player_entity) = players.get(other_entity) {
                    if player_entity.0 != bullet.shooter_id {
                        // health.current -= bullet.damage;
                        commands.entity(bullet_entity).despawn();
                        // commands.entity(player_entity).despawn();
                        commands.entity(player_entity.1.get()).despawn_recursive();
                    }
                }
            }
        }
    }
}
