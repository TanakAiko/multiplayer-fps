use bevy::prelude::*;

// Camera pour le personnage (Ce que le joueur peut voir de son propre corps)
#[derive(Debug, Component)]
pub struct ViewModel;

// Camera pour la nature (Ce que le joueur peut voir du monde)
#[derive(Debug, Component)]
pub struct WorldModelCamera;


#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            Vec2::new(0.003, 0.002)
        )
    }
}