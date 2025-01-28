use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionEvent;

use crate::client::components::player_component::Velocity;



pub fn collider_detect_world(
    mut collision_events: EventReader<CollisionEvent>,
    mut _query: Query<(&mut Velocity, &Transform)>
) {
    for collision_event in collision_events.read() {
        println!("Collision event detected - {:?}", collision_event);
        match collision_event {
            CollisionEvent::Started(e1, e2, flags) => {
                println!("Collision started between entities {:?} and {:?} - {:?}", e1, e2, flags);
            },
            CollisionEvent::Stopped(e1, e2, _) => {
                println!("Collision stopped between entities {:?} and {:?}", e1, e2);
            },
            _ => {
                println!("Collision event detected - {:?}", collision_event);
            }

        }
    }
}