use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

pub fn move_enemy(
    name: String,
    mut position: Vec3,
    rotation: Quat,
    asset_server: Res<AssetServer>,
    mut query: Query< (&Enemy, &mut Transform, &mut AnimationPlayer)>,
) {
    position.y = -0.5; // Ajustement de la taille de l'ennemi
    for ( enemy, mut transform, mut animation_player)in query.iter_mut() {
        if enemy.name == name {
            let old_position = transform.translation;
            let distance = old_position.distance(position); // Calcul de la distance entre la position actuelle et la nouvelle position

            if distance > 0.01 { // Commence a bouger consequemment
                // On joue l'animation de déplacement
                let walk_animation = AnimationGraph::from_clip(
                    asset_server.load(GltfAssetLabel::Animation(2).from_asset("fps_enemy.gltf")),
                );
                animation_player.play(walk_animation.1.clone());
            } else {
                // On joue l'animation Idle
                // animation_player.play("Idle".to_string());
            }

            transform.translation = position;
            // transform.rotation = rotation.conjugate();
            let (yaw, _, _z) = rotation.to_euler(EulerRot::YXZ);
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);
        }
    }
}
