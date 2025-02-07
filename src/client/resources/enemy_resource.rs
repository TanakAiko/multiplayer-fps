use bevy::prelude::*;

use crate::{client::components::enemy_component::Enemy, common::types::protocol::CommonPlayer};

#[derive(Debug, Resource)]
pub struct EnemyResource {
    pub enemies : Vec<Enemy>
}

impl EnemyResource {
    pub fn new(nemies: Vec<CommonPlayer>) -> Self {
        let mut enemies = Vec::new();
        for enemy in nemies {
            enemies.push(Enemy{
                name: enemy.name,
                position: enemy.position,
                ..Default::default()
            });
        }

        EnemyResource {
            enemies
        }
    }
}