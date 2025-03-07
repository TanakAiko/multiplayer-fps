use bevy::prelude::*;

use crate::client::components::enemy_component::{Enemy, EnemyMovement};

/* pub fn move_enemy(
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
 */

const LERP_SPEED: f32 = 15.0; // Ajustez cette valeur pour contrôler la vitesse de l'interpolation

pub fn move_enemy(
    name: String,
    mut position: Vec3,
    rotation: Quat,
    mut enemy_query: Query<(&Parent, &Enemy, &mut EnemyMovement)>,
    mut parent_query: Query<&mut Transform>,
    time: Res<Time>,
) {
    position.y = 0.;

    for (parent, enemy, mut movement) in enemy_query.iter_mut() {
        if enemy.name == name {
            if let Ok(mut parent_transform) = parent_query.get_mut(parent.get()) {
                // Mettre à jour la position cible
                movement.target_position = position;
                
                // Calculer la nouvelle position avec lerp
                movement.current_position = lerp(
                    movement.current_position,
                    movement.target_position,
                    LERP_SPEED * time.delta_secs()
                );

                parent_transform.translation = movement.current_position;

                // Rotation (garder seulement Yaw)
                let (yaw, _, _) = rotation.to_euler(EulerRot::YXZ);
                let additional_rotation = Quat::from_rotation_y(std::f32::consts::PI);
                parent_transform.rotation = additional_rotation * Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);
            }
        }
    }
}

// Fonction d'interpolation linéaire
fn lerp(start: Vec3, end: Vec3, t: f32) -> Vec3 {
    start + (end - start) * t.clamp(0.0, 1.0)
}