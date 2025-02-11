use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

pub fn move_enemy(
    name: String,
    mut position: Vec3,
    rotation: Quat,
    enemy_query: Query<(&Parent, &Enemy)>, // 🔹 Récupère l'ennemi et son parent
    mut parent_query: Query<&mut Transform>,
) {
    position.y = 0.; // Ajustement de la taille de l'ennemi

    for (parent, enemy) in enemy_query.iter() {
        if enemy.name == name {
            if let Ok(mut parent_transform) =
                parent_query.get_mut(parent.get())
            {
                parent_transform.translation = position;

                // 🔹 Appliquer la rotation (conserver seulement Yaw)
                let (yaw, _, _) = rotation.to_euler(EulerRot::YXZ);
                let additional_rotation = Quat::from_rotation_y(std::f32::consts::PI); // 🔹 Rotation de 180° (PI radians)

                parent_transform.rotation =
                    additional_rotation * Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);
            }
        }
    }
}
