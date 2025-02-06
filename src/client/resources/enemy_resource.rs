use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

#[derive(Debug, Resource)]
pub struct EnemyResource {
    pub enemies : Vec<Enemy>
}
