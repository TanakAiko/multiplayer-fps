use bevy::prelude::*;

#[derive(Debug,Component)]
pub struct Bullet {
    pub speed: f32,
    pub damage: f32,
    pub lifetime: Timer,
    pub shooter_id: Entity,
}

#[derive(Component)]
pub struct BulletDirection(pub Vec3);
