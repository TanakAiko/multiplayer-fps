use std::time::Instant;

use bevy::prelude::*;

use crate::{
    client::{components::player_component::Player, resources::network_resource::NetworkResource},
    common::types::protocol::Message,
};

const UPDATE_FREQUENCY: f32 = 1.0 / 20.0; // 20 Hz ;

pub fn send_player_updates(
    mut network: ResMut<NetworkResource>,
    query: Query<(&Transform, &Player)>,
) {
    if network.last_sent.elapsed().as_secs_f32() < UPDATE_FREQUENCY {
        return;
    }

    if let Ok((transform, _)) = query.get_single() {

        let update = Message::PlayerUpdateSending {
            position: transform.translation,
            rotation: transform.rotation,
        };

        let encoded = bincode::serialize(&update).unwrap();
        if let Err(e) = network.socket.try_send(&encoded) {
            error!("Erreur d'envoi: {}", e);
        }
    }

    network.last_sent = Instant::now();
}
