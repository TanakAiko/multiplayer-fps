use bevy::{color::palettes::tailwind, prelude::*};
use crate::client::resources::world_resource::MazeResource;


pub fn spawn_map(
    mut commands: Commands,
    window_query: Query<&Window>,
    maze_resource: Res<MazeResource>,
) {

    let maze = maze_resource.grid.clone();


    let shape_size = 10.0;
    let minimap_margin = 10.0;

    let window = window_query.single();
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        Camera2d::default(),
        Camera {
            order: 1,
            ..Default::default()
        },
    ));


    for (z, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let position = Vec3::new(
                -window_width / 2.0 + shape_size / 2.0 + minimap_margin + x as f32 * shape_size, 
                -window_height / 2.0 + shape_size / 2.0 + minimap_margin + z as f32 * shape_size, 
                0.
            );

            match cell {
                'b' => {
                    commands.spawn((
                        Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(shape_size, shape_size)),
                            ..Default::default()
                        },
                        Transform::from_xyz(position.x, position.y, position.z),
                    ));
                }
                'c' => {
                    commands.spawn((
                        Sprite {
                            color: Color::from(tailwind::ORANGE_400),
                            custom_size: Some(Vec2::new(shape_size, shape_size)),
                            ..Default::default()
                        },
                        Transform::from_xyz(position.x, position.y, position.z),
                    ));
                }
                _ => {
                    commands.spawn((
                        Sprite {
                            color: Color::BLACK,
                            custom_size: Some(Vec2::new(shape_size, shape_size)),
                            ..Default::default()
                        },
                        Transform::from_xyz(position.x, position.y, position.z),
                    ));
                }
            }
        }
    }
    
}