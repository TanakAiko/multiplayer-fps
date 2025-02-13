use bevy::{prelude::*, render::view::RenderLayers};

use crate::client::{
    components::player_component::MiniMapPlayer, resources::player_resource::PlayerResource,
};

// ✅ Si ta minimap utilise RenderLayers, ajoute un identifiant spécifique
const MINIMAP_LAYER: RenderLayers = RenderLayers::layer(0);

pub fn spawn_mini_map_player(mut commands: Commands, res_player: &Res<PlayerResource>) {
    println!("Spawn mini map player");

    let shape_size = 10.0 * 0.5;

    commands.spawn((
        Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(shape_size, shape_size)),
            ..Default::default()
        },
        Transform::from_xyz(res_player.position.x, res_player.position.y, 0.),
        MiniMapPlayer,
        Visibility::Visible,
        MINIMAP_LAYER,
    ));
}

pub fn update_minimap_player(
    mut minimap_query: Query<&mut Transform, With<MiniMapPlayer>>, // 🔹 Récupère l'icône du joueur
    window_query: Query<&Window>,
    res_player: Res<PlayerResource>,
) {
    if let Ok(mut minimap_transform) = minimap_query.get_single_mut() {
        println!("res_player.position: {}", res_player.position);
        let window = window_query.single();
        let window_width = window.width();
        let window_height = window.height();

        println!(
            "window_height: {} || window_width: {}",
            window_height, window_width
        );

        let minimap_x = (res_player.position.x * 5.0) - 525.; // 🔄 Échelle arbitraire (à ajuster)
        let minimap_y = (res_player.position.z * 5.0) - 260.;

        minimap_transform.translation.x = minimap_x;
        minimap_transform.translation.y = minimap_y;
    }
}
