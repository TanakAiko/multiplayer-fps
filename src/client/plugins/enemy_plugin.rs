use bevy::prelude::*;

use crate::client::systems::enemy::setup_enemy::spawn_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_enemy);
    }
}