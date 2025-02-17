use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{
    client::{
        components::{enemy_component::Enemy, player_component::Player}, resources::network_resource::NetworkResource,
        systems::{common::remove_the_dead::despawn_the_dead, enemy::move_enemy::move_enemy},
    },
    common::types::protocol::Message,
};

pub fn handle_network_messages(
    network: Res<NetworkResource>,
    mut commands: Commands,
    enemy_query: Query<(&Parent, &Enemy)>,
    enemy_query_2: Query<(&Parent, &Enemy), With<Enemy>>,
    // query_player: Query<(&Parent, &Player), With<Player>>,
    query_player: Single<(Entity, &Player)>,
    mut exit_writer: EventWriter<AppExit>,
    parent_query: Query<&mut Transform>,
) {
    let mut buf = vec![0; 1024];
    match network.socket.try_recv(&mut buf) {
        Ok(len) => {
            if let Ok(message) = bincode::deserialize(&buf[..len]) {
                match message {
                    Message::Leave => {
                        info!("Un joueur a quitté le serveur");
                    }
                    Message::PlayerUpdateReceiving {
                        name,
                        position,
                        rotation,
                    } => {
                        move_enemy(
                            name,
                            position,
                            rotation,
                            enemy_query,
                            parent_query,
                        );
                    }
                    Message::GameOver { loser_name } => {
                        println!("Game Over, {} a perdu !", loser_name);
                        // despawn_ennemy(commands.reborrow(), loser_name, &enemy_query_2);
                        despawn_the_dead(commands.reborrow(), loser_name, &enemy_query_2, &query_player);
                        // exit_writer.send(AppExit::Success);
                    }
                    Message::Win => {
                        println!("Nahhh, I'd Win !!! 😎🔥");
                        exit_writer.send(AppExit::Success);
                    }
                    Message::Lose => {
                        println!("You Loserrrrr ❌");
                        exit_writer.send(AppExit::Success);
                    }
                    _ => todo!(),
                }
            }
        }
        Err(_) => {} // Ignore errors
    }
}
