use crate::client::{
    components::world_component::WallModel, resources::world_resource::MazeResource,
};
use bevy::{color::palettes::tailwind, prelude::*};
use bevy_rapier3d::prelude::*;

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze_resource: Res<MazeResource>,
) {
    let maze = maze_resource.grid.clone();
    let maze_width = maze_resource.width as f32;
    let maze_height = maze_resource.height as f32;

    let tile_size = 0.4; // Taille d'une tuile
    let spacing = 5.; // Espace entre les murs

    let floor_size = Vec2::new(
        maze_width * tile_size * spacing,
        maze_height * tile_size * spacing,
    );

    let floor_mesh = meshes.add(Plane3d::new(Vec3::Y, floor_size));
    let floor_material = materials.add(Color::from(tailwind::GRAY_400));
    commands.spawn((
        Mesh3d(floor_mesh),
        MeshMaterial3d(floor_material),
        Collider::cuboid(floor_size.x, 0.08, floor_size.y),
        RigidBody::Fixed,
        // ActiveEvents::COLLISION_EVENTS,
        Transform::from_xyz(
            -(maze_width * tile_size * spacing) / 2.0,
            -0.1,
            -(maze_height * tile_size * spacing) / 2.0,
        ),
    ));

    let wall_mesh = meshes.add(Cuboid::new(tile_size * spacing, 5.0, tile_size * spacing)); // Ajuste la taille du mur
    let wall_material = materials.add(Color::from(tailwind::ORANGE_400));

    for (z, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let position = Vec3::new(
                x as f32 * tile_size * spacing - (maze_width * tile_size * spacing) / 2.0,
                0.,
                z as f32 * tile_size * spacing - (maze_height * tile_size * spacing) / 2.0,
            );
            match cell {
                'b' => {
                    commands.spawn((
                        Mesh3d(wall_mesh.clone()),
                        MeshMaterial3d(wall_material.clone()),
                        Transform::from_xyz(position.x, position.y, position.z),
                        RigidBody::Fixed,
                        ActiveEvents::COLLISION_EVENTS,
                        Collider::cuboid(
                            (tile_size * spacing) / 2.,
                            5.0 / 2.,
                            (tile_size * spacing) / 2.,
                        ),
                        WallModel,
                    ));
                }
                'f' => {}
                _ => {}
            }
        }
    }
}
