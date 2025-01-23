use bevy::prelude::*;
use serde_json::to_string;

use crate::client::systems::world::load_json_world::{self, load_maze_from_json};

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let maze = load_maze_from_json("maze.json");
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(30.)));
    let mur = meshes.add(Cuboid::new(1.0, 4.0, 0.2));
    let material = materials.add(Color::WHITE);
    commands
        .spawn((
            Mesh3d(floor.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(0.2, 0., 0.5),
        ));

    // Définir les dimensions des tuiles
    let tile_size = 1.0;

    for (z, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            // Calculer la position 3D
            let position = Vec3::new(x as f32 * tile_size, 0.0, z as f32 * tile_size);

            match cell {
                'b' => {
                    // Mur (cube solide)
                    commands
                    .spawn((
                        Mesh3d(mur.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_translation(position),
                    ));
                }
                'c' => {
                    // Sol (chemin libre)
                }
                'm' => {
                    // Obstacle ou chemin particulier
                    
                }
                _ => {}
            }
        }
    }
}