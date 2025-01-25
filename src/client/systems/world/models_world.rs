use bevy::{color::palettes::tailwind, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::client::systems::world::load_json_world::{self, load_maze_from_json};

/* pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let maze = load_maze_from_json("maze.json");
    let max_len = if maze.len() > maze[0].len() {
        maze.len()
    } else {
        maze[0].len()
    };
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(max_len as f32)));
    let wall = meshes.add(Cuboid::new(0.3, 3.0, 0.3));
    let material = materials.add(Color::from(tailwind::GRAY_400));
    let wall_material = materials.add(Color::from(tailwind::ORANGE_400));
    // setup le sol
    commands.spawn((
        Mesh3d(floor.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(-1.*max_len as f32 +1., -1., -1.*max_len as f32 +1.), // ajustement manuel
    ));

    let tile_size = 0.3;

    for (z, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            // Calculer la position 3D
            let position = Vec3::new(x as f32 * tile_size, 0.0, z as f32 * tile_size);

            match cell {
                'b' => {
                    println!("x : {}, z : {}", position.x, position.z);
                    // Mur (cube solide)
                    commands.spawn((
                        Mesh3d(wall.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(position.x, 0.5, -1.0*position.z),
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
  */

  pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let maze = load_maze_from_json("maze.json");

    let maze_width = maze[0].len();
    let maze_height = maze.len();

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
