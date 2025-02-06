use bevy::prelude::*;

use crate::{
    client::{components::enemy_component::Enemy, resources::network_resource::NetworkResource},
    common::types::protocol::Message,
};

use super::{move_enemy::move_enemy, setup_enemy::spawn_enemy};

pub fn handle_network_messages(
    network: Res<NetworkResource>,
    asset_server: Res<AssetServer>,
    commands: Commands,
    query: Query< (&Enemy, &mut Transform)>,
) {
    let mut buf = vec![0; 1024];
    match network.socket.try_recv(&mut buf) {
        Ok(len) => {
            if let Ok(message) = bincode::deserialize(&buf[..len]) {
                match message {
                    Message::Join { name } => {
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
                        move_enemy(name, position, rotation, query)
                    }
                    _ => todo!(),
                }
            }
        }
        Err(_) => {} // Ignore errors
    }
}
