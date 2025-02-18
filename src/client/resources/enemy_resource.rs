use bevy::prelude::*;

use crate::{client::components::enemy_component::Enemy, common::types::protocol::CommonPlayer};

#[derive(Debug, Resource)]
pub struct EnemyResource {
    pub enemies : Vec<Enemy>,
    pub dead_players: Vec<String>,
}

impl EnemyResource {
    pub fn new(common_players: Vec<CommonPlayer>) -> Self {
        let mut enemies = Vec::new();
        for enemy in common_players {
            enemies.push(Enemy{
                name: enemy.name,
                position: enemy.position,
                ..Default::default()
            });
        }

        EnemyResource {
            enemies,
            dead_players: Vec::new(),
        }
    }
}


#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EnemyState {
    #[default]
    Idle,
    Run,
    Shoot,
    Death,
    GunPointing,
}
