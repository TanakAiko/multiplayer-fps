use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

pub fn move_enemy(
    name: String,
    mut position: Vec3,
    rotation: Quat,
    mut query: Query< (&Enemy, &mut Transform)>,
) {
    position.y = -0.5; // Ajustement de la taille de l'ennemi
    for ( enemy, mut transform)in query.iter_mut() {
        if enemy.name == name {
            transform.translation = position;
            // transform.rotation = rotation.conjugate();
            let (_, y, _z) = rotation.to_euler(EulerRot::XYZ);
            transform.rotation = Quat::from_euler(EulerRot::XYZ, 0., -y + std::f32::consts::PI, 0.);
        }
    }
}
