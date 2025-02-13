use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Player {
    pub name: String,
    pub position: Vec3,
    pub shoot_timer: Timer,
    // pub health: f32
}

#[derive(Debug, Component)]
pub struct MiniMapPlayer;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

#[derive(Component)]
pub struct PlayerStep;

#[derive(Component)]
pub struct PlayerShoot;
