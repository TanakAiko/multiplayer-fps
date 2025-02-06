use std::time::Instant;

use bevy::prelude::*;

use crate::{client::{components::player_component::{Player, Velocity}, resources::network_resource::NetworkResource}, common::types::protocol::Message};


pub fn send_player_updates(
    mut network : ResMut<NetworkResource>,
    query: Query<(&Transform, &Player), With<Velocity>>,
) {
    const UPDATE_FREQUENCY: f32 = 1.0 / 20.0; // 20 Hz ;

    if network.last_sent.elapsed().as_secs_f32() < UPDATE_FREQUENCY {
        return;
    }

    if let Ok((transform, _)) = query.get_single() {
        let update = Message::PlayerUpdateSending { 
            position: transform.translation, 
            rotation: transform.rotation, 
        };

        let encoded =bincode::serialize(&update).unwrap();
        if let Err(e) = network.socket.try_send(&encoded) { // Envoi de nos informations aux serveurs 
            error!("Erreur d'envoi: {}", e);
        }
    }

    network.last_sent = Instant::now();


}