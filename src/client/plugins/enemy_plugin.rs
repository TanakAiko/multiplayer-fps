use bevy::prelude::*;

use crate::client::{
    resources::{
        animation_resource::{AnimationResource, AnimationState},
        enemy_resource::EnemyState,
    },
    systems::enemy::{
        handle_animation_enemy::{handle_enemy_animations, update_enemy_state},
        setup_enemy::spawn_all_enemies,
    },
};

// use crate::client::systems::enemy::setup_enemy::spawn_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EnemyState>()
            .init_resource::<AnimationState>()
            .init_resource::<AnimationResource>()
            .add_systems(Startup, spawn_all_enemies)
            .add_systems(
                Update,
                (update_enemy_state, handle_enemy_animations).chain(),
            );
    }
}
