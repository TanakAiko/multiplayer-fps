
use bevy::{prelude::*, utils::HashMap};

use crate::client::{components::enemy_component::Enemy, resources::enemy_resource::EnemyState};

pub fn handle_enemy_animations(
    mut query: Query<(&Enemy, &Children, &mut AnimationPlayer)>,
    mut prev_states: Local<HashMap<String, EnemyState>>,
) {
    for (enemy, _children, mut animation_player) in query.iter_mut() {
        let prev_state = prev_states.get(&enemy.name).unwrap_or(&EnemyState::Idle);
        println!("prev_state: {:?}, current_state: {:?}", prev_state, enemy.current_state);
        if prev_state != &enemy.current_state {
            let index = match enemy.current_state {
                EnemyState::Idle => 4,
                EnemyState::Run => 16,
                EnemyState::GunPointing => 6,
                EnemyState::Death => 0,
                EnemyState::Shoot => 1,
            };
            animation_player.play(AnimationNodeIndex::new(index));

            prev_states.insert(enemy.name.clone(), enemy.current_state.clone());
        }
    }
}


pub fn update_enemy_state(
    mut enemies: Query<(&mut Enemy, &Transform)>,
) {
    for (mut enemy, transform) in enemies.iter_mut() {
        let velocity = (transform.translation - enemy.position).length();
        enemy.current_state = if velocity > 0.1 {
            EnemyState::Run
        } else {
            EnemyState::GunPointing
        };
        

    }
}
