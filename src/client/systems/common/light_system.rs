use bevy::{prelude::*, render::view::RenderLayers};

use crate::client::{resources::world_resource::MazeResource, systems::camera::setup_camera::{DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER}};




 pub fn spawn_light(
    mut commands: Commands,
    maze_resource: Res<MazeResource>,
     // Si vous voulez placer les lumières en fonction du labyrinthe
) {
    // les dimensions du labyrinthe
    let maze_width = maze_resource.width as f32;
    let maze_height = maze_resource.width as f32;

    println!("Width: {}, Height: {}", maze_width, maze_height);

    let spacing = 10.0; // Distance entre chaque lumière

    // Créer une grille de lumières
    for x in 0..=(maze_width as i32 / spacing as i32) {
        for z in 0..=(maze_height as i32 / spacing as i32) {
            commands.spawn((
                PointLight {
                    intensity: 1000000.0,
                    shadows_enabled: true,
                    range: 30.0, // Portée de la lumière
                    radius: 2.0, // Taille de la source
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * spacing - maze_width / 2.0,
                    5.0, // Hauteur des lumières
                    z as f32 * spacing - maze_height / 2.0,
                ),
                RenderLayers::from_layers(&[DEFAULT_RENDER_LAYER, VIEW_MODEL_RENDER_LAYER]),
            ));
        }
    }
}
 