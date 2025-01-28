use bevy::{color::palettes::tailwind, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::client::resources::world_resource::MazeResource;

  pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_resource: Res<MazeResource>
) {
    // let maze = load_maze_from_json(commands);

    let maze = maze_resource.grid.clone();
    let maze_width = maze_resource.width as f32;
    let maze_height = maze_resource.height as f32;

    let tile_size = 1.0; // Taille d'une tuile (plus facile à ajuster ici)

    // Dimensions du sol basées sur la taille du labyrinthe
    let floor_size = Vec2::new(
        maze_width as f32 * tile_size,
        maze_height as f32 * tile_size,
    );

    // Ajouter un sol centré
    let floor_mesh = meshes.add(Plane3d::new(Vec3::Y, floor_size));
    let floor_material = materials.add(Color::from(tailwind::GRAY_400));
    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
        Collider::cuboid(floor_size.x, 0.1, floor_size.y),
        RigidBody::Fixed,
        Transform::from_xyz(
            -(maze_width as f32 * tile_size) / 2.0,
            -0.5,
            -(maze_height as f32 * tile_size) / 2.0,
        ),
    ));

    // Préparer le mur
    let wall_mesh = meshes.add(Cuboid::new(tile_size * 1., 3.0, tile_size * 1.0)); // Ajuste la taille
    let wall_material = materials.add(Color::from(tailwind::ORANGE_400));
    
    // Itérer sur le labyrinthe pour placer les murs
    for (z, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            // Calculer la position de la tuile (centrée autour de l'origine)
            let position = Vec3::new(
                x as f32 * tile_size - (maze_width as f32 * tile_size) / 2.0,
                1.0,
                z as f32 * tile_size - (maze_height as f32 * tile_size) / 2.0,
            );

            match cell {
                'b' => {
                    // Placer un mur
                    commands.spawn((
                        Mesh3d(wall_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(position.x, position.y, position.z),
                        RigidBody::Fixed,  
                        Collider::cuboid(tile_size * 1., 3.0, tile_size * 1.0),
                        CollisionGroups::new(
                            Group::from_bits_truncate(0b0010),
                            Group::from_bits_truncate(0b0001),
                        ),
                    ));
                }
                'c' => {
                    // Case vide, ne rien faire
                }
                _ => {
                    // Cas non reconnu (peut être ignoré)
                }
            }
        }
    }
}
