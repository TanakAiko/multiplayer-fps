use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    client::{
        components::{
            bullet::{Bullet, BulletDirection},
            camera_component::CameraSensitivity,
            enemy_component::Enemy,
            player_component::{Player, PlayerShoot},
        },
        resources::network_resource::NetworkResource,
        systems::common::remove_the_dead::despawn_the_dead,
    },
    common::types::protocol::Message,
};

use super::step::playsoundshoot;

pub fn player_shooting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, Entity)>,
    camera_query: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
    music_controller: Query<&AudioSink, With<PlayerShoot>>,
) {
    if let (Ok((mut player, entity)), Ok(camera_transform)) =
        (player_query.get_single_mut(), camera_query.get_single())
    {
        player.shoot_timer.tick(time.delta());

        let mut is_shooting = false;
        if keyboard.pressed(KeyCode::Space) {
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
            is_shooting = true;
            player.shoot_timer.reset();
        }
        playsoundshoot(&music_controller, is_shooting);
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
            Mesh3d(meshes.add(Mesh::from(Sphere::new(0.05)))),
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
    enemies_query: Query<(Entity, &Parent, &Enemy), With<Enemy>>,
    enemies_query_2: Query<(&Parent, &Enemy), With<Enemy>>,
    // player_query: Query<(&Parent, &Player), With<Player>>,
    player_query: Single<(Entity, &Player)>,
    mut collision_events: EventReader<CollisionEvent>,
    network: ResMut<NetworkResource>,
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
                if let Ok(player_entity) = enemies_query.get(other_entity) {
                    if player_entity.0 != bullet.shooter_id {
                        // health.current -= bullet.damage;
                        commands.entity(bullet_entity).despawn();

                        // commands.entity(player_entity.1.get()).despawn_recursive();
                        despawn_the_dead(
                            commands.reborrow(),
                            player_entity.2.name.clone(),
                            &enemies_query_2,
                            &player_query,
                        );
                        let game_over = Message::GameOver {
                            loser_name: player_entity.2.name.clone(),
                        };

                        let encoded = bincode::serialize(&game_over).unwrap();
                        if let Err(e) = network.socket.try_send(&encoded) {
                            error!("Erreur d'envoi: {}", e);
                        }
                    }
                }
            }
        }
    }
}
