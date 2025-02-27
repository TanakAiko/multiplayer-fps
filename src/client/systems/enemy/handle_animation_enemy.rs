
use std::time::Duration;

use bevy::prelude::*;

use crate::client::{components::enemy_component::Enemy, resources::animation_resource::{AnimationResource, AnimationState}};

pub fn handle_enemy_animations(
    animations: Res<AnimationResource>,
    animation_state: Res<AnimationState>,
    mut players: Query<(Entity, &mut AnimationPlayer)>,
    mut commands: Commands,
) {
    println!("Handle enemy animations");
    for (entity, mut player) in &mut players {
        info!(" --------------------------------------------------- Handle enemy animations");
        let mut transitions = AnimationTransitions::new();
        
        // Démarrer avec l'animation idle
        transitions
            .play(&mut player, animations.animations[animation_state.current_animation], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph_handle.clone()))
            .insert(transitions);
    }
}


/* pub fn update_enemy_state(
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
} */

pub fn update_enemy_state(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions, &mut Enemy, &Transform)>,
    animations: Res<AnimationResource>,
    mut animation_state: ResMut<AnimationState>,
    time: Res<Time>,
) {
    println!("Update enemy state");
    for (mut player, mut transitions, mut enemy, transform) in &mut animation_players {
        // Exemple de logique pour changer d'animation selon l'état
        // 2 = gun shooting
        // 0 = idle
        // 1 = run
        // 3 = death
        // 4 = hit
        let mut new_animation = 1;
        println!("--------------------------------=-Distance: {}", transform.translation.distance(enemy.position));
        if transform.translation.distance(enemy.position) != 0. {
            println!("Runnniiiiiiiiiiiiiiiiinnnnnnnnnnnnnnnnnngggggggggggggggggggggggggggggg");
            new_animation = 2;
            enemy.animation_timer.reset();
        } else {
            // Si on ne bouge pas, on vérifie le timer
            enemy.animation_timer.tick(time.delta());
            
            // Si le timer n'est pas fini, on maintient l'animation de course
            if !enemy.animation_timer.finished() {
                new_animation = 2;
            }
        }
        enemy.position = transform.translation;
        
        // Changer l'animation si nécessaire
        if animation_state.current_animation != new_animation {
            println!("/////////////////////////////////////");
            animation_state.current_animation = new_animation;
            transitions.play(
                &mut player,
                animations.animations[new_animation],
                Duration::from_millis(300),
            ).repeat();
        }
    }
}
