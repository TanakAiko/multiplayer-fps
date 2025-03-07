use bevy::prelude::*;

use crate::client::{
    components::{enemy_component::Enemy, player_component::Player},
    systems::page_system::setup_game_over_system::spawn_game_over_ui,
};

pub fn despawn_the_dead(
    mut commands: Commands,
    all_dead_players: &Vec<String>,
    query: &Query<(&Parent, &Enemy), With<Enemy>>,
    // query_player: &Query<(&Parent, &Player), With<Player>>,
    query_player: &Single<(Entity, &Player)>,
) {
    let player_name = query_player.1.name.clone();
    let _player_parent = query_player.0;

    for name in all_dead_players {
        if player_name == *name {
            // commands.entity(player_parent).despawn_recursive();
            spawn_game_over_ui(commands.reborrow());
            println!("You Loserrrrr ❌");
            // Attendre un peu avant de quitter
            std::thread::sleep(std::time::Duration::from_secs(2));
            std::process::exit(0);
        }

        println!(
            "all_dead_players.len(): {}  ||  query.iter().count(): {}",
            all_dead_players.len(),
            query.iter().count()
        );
        
        if all_dead_players.len() >= query.iter().count() {
            // spawn_game_over_ui(commands.reborrow());
            println!("Nahhh, I'd Win !!! 😎🔥");
            // Attendre un peu avant de quitter
            std::thread::sleep(std::time::Duration::from_secs(2));
            std::process::exit(0);
        }

        for (parent, enemy) in query.iter() {
            if enemy.name == *name {
                commands.entity(parent.get()).despawn_recursive();
            }
        }
    }
}
