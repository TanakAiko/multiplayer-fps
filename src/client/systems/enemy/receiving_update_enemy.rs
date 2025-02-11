use bevy::app::AppExit;
use bevy::prelude::*;

use crate::{
    client::{
        components::enemy_component::Enemy, resources::network_resource::NetworkResource,
        systems::enemy::move_enemy::move_enemy,
    },
    common::types::protocol::Message,
};

pub fn handle_network_messages(
    network: Res<NetworkResource>,
    _commands: Commands,
    enemy_query: Query<( &Parent, &Enemy)>,
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
