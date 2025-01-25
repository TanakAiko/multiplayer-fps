use bevy::prelude::*;
use bevy_rapier3d::prelude::CollisionEvent;

use crate::client::systems::player::move_player::Velocity;

pub fn collider_detect_world(
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(&mut Velocity, &Transform)>
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                println!("Collision started between entities {:?} and {:?}", e1, e2);
            },
            CollisionEvent::Stopped(e1, e2, _) => {
                println!("Collision stopped between entities {:?} and {:?}", e1, e2);
            }
            
        }
    }
}