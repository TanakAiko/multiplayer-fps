use crate::{
    client::{components::flag_component::Flag, resources::network_resource::NetworkResource},
    common::types::protocol::Message,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionEvent;

pub fn collider_detect_world(
    mut collision_events: EventReader<CollisionEvent>,
    network: ResMut<NetworkResource>,
    flag_query: Query<Entity, With<Flag>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, flags) => {
                println!(
                    "Collision started between entities {:?} and {:?} -- {:?}",
                    e1, e2, flags
                );
                let has_flag = flag_query.get(*e1).is_ok() || flag_query.get(*e2).is_ok();
                if has_flag {
                    println!("🚩 Flag is captured ! {:?}", (e1, e2));
                    let encoded = bincode::serialize(&Message::GameOver).unwrap();
                    if let Err(e) = network.socket.try_send(&encoded) {
                        error!("Erreur d'envoi: {}", e);
                    }
                } /*  else {
                      println!("Collision normale entre {:?} et {:?}", e1, e2);
                  } */
            }
            CollisionEvent::Stopped(_e1, _e2, _flags) => {
                // println!(
                //     "Collision stopped between entities {:?} and {:?} -- {:?}",
                //     e1, e2, flags
                // );
            }
        }
    }
}
