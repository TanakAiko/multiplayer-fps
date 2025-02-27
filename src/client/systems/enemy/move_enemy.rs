use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

pub fn move_enemy(
    name: String,
    mut position: Vec3,
    rotation: Quat,
    mut enemy_query: Query<(&mut Transform, &Enemy)>, // 🔹 Récupère l'ennemi et son parent
    // mut _parent_query: Query<&mut Transform>,
) {
    position.y = 0.; // Ajustement de la taille de l'ennemi

    for (mut parent, enemy) in enemy_query.iter_mut() {
        if enemy.name == name {
            parent.translation = position;

            // 🔹 Appliquer la rotation (conserver seulement Yaw)
            let (yaw, _, _) = rotation.to_euler(EulerRot::YXZ);
            let additional_rotation = Quat::from_rotation_y(std::f32::consts::PI); // 🔹 Rotation de 180° (PI radians)

            parent.rotation =
                additional_rotation * Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);
        }
    }
}
