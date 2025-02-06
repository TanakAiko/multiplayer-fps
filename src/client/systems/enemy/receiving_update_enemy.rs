use bevy::prelude::*;

use crate::{
    client::{components::enemy_component::Enemy, resources::network_resource::NetworkResource},
    common::types::protocol::Message,
};

use super::setup_enemy::spawn_enemy;

pub fn handle_network_messages(
    network: Res<NetworkResource>,
    asset_server: Res<AssetServer>,
    commands: Commands,
    mut query: Query<(&Enemy, &mut Transform)>,
) {
    let mut buf = vec![0; 1024];
    match network.socket.try_recv(&mut buf) {
        Ok(len) => {
            if let Ok(message) = bincode::deserialize(&buf[..len]) {
                match message {
                    Message::Join { name } => {
                        info!("Un joueur a rejoint le serveur");
                        spawn_enemy(name, commands, asset_server);
                    }
                    Message::Leave => {
                        info!("Un joueur a quitté le serveur");
                    }
                    Message::PlayerUpdateReceiving {
                        name,
                        position,
                        rotation,
                    } => {
                        println!("------------------------------------------------");
                        // move_enemy(name, position, rotation, query)

                        for (enemy, mut transform) in query.iter_mut() {
                            println!("{} ======== {}", enemy.name, name);
                            
                            if enemy.name == name {
                                // Réinitialiser les forces physiques
                                println!("**********************************************");
                                println!("position {}", position);
                                println!("rotation {}", rotation);
                                transform.translation = position;
                                transform.rotation = rotation;
                                println!("transform.translation {}", transform.translation);
                                println!("transform.rotation {}", transform.rotation);
                                println!("**********************************************");

                            }
                        }
                    }
                    _ => todo!(),
                }
            }
        }
        Err(_) => {} // Ignore errors
    }
}
