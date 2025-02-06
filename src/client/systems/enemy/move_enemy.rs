use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

pub fn move_enemy(
    name: String,
    position: Vec3,
    rotation: Quat,
    mut query: Query< (&Enemy, &mut Transform)>,
) {
    for ( enemy, mut transform)in query.iter_mut() {
        println!("**********************************************");
        println!("{} ======== {}", enemy.name, name);
        if enemy.name == name {
            transform.translation = position;
            transform.rotation = rotation;
        }
    }
}
