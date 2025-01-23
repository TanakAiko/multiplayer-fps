use bevy::prelude::*;
use serde_json::to_string;

use crate::client::systems::world::load_json_world::{self, load_maze_from_json};

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // println!("{:?}", load_maze_from_json("./src/client/maze.json"));

    /* let cube= meshes.add(Cuboid::new(2., 0.5, 1.));
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.)));
    let material = materials.add(Color::WHITE);


    commands.spawn((
        Mesh3d(floor),
        MeshMaterial3d(material.clone())
    ));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.75, 2., 0.0),
    )); */
}