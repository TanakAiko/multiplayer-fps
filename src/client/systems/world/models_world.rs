use bevy::prelude::*;

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube= meshes.add(Cuboid::new(2., 0.5, 1.));
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
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));
}