use bevy::prelude::*;

use crate::client::components::enemy_component::Enemy;

pub fn move_enemy(
    name: String,
    mut position: Vec3,
    rotation: Quat,
    asset_server: Res<AssetServer>,
    enemy_query: Query<(Entity, &Parent, &Enemy)>, // 🔹 Récupère l'ennemi et son parent
    mut parent_query: Query<(&mut Transform, &mut AnimationPlayer)>,
) {
    position.y = 0.; // Ajustement de la taille de l'ennemi
    for (_yenemy_entity, parent, enemy) in enemy_query.iter() {
        if enemy.name == name {
            if let Ok((mut parent_transform, mut animation_player)) = parent_query.get_mut(parent.get()) {
                // 🔹 Accède au transform du parent
                
                let old_position = parent_transform.translation;
                let distance = old_position.distance(position); // 🔹 Distance entre ancienne et nouvelle position

                if distance > 0.01 {
                    // Commence à bouger
                    let walk_animation = AnimationGraph::from_clip(
                        asset_server.load(GltfAssetLabel::Animation(2).from_asset("fps_enemy.gltf")),
                    );
                    // 🔹 Si besoin, jouer animation
                    animation_player.play(walk_animation.1.clone());
                    // animation_player.play(walk_animation.1.clone());
                } else {
                    // Animation Idle si immobile
                    // animation_player.play("Idle".to_string());
                }

                parent_transform.translation = position;

                // 🔹 Appliquer la rotation (conserver seulement Yaw)
                let (yaw, _, _) = rotation.to_euler(EulerRot::YXZ);
                parent_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, 0., 0.);
            }
        }
    }
}
