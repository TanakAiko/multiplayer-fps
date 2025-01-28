use bevy::prelude::*;

use crate::client::resources::world_resource::MazeResource;

pub fn load_maze_from_json(
    mut commands: Commands,
) {
    let json_data = include_str!("maze.json");
    let grid: Vec<Vec<String>> = serde_json::from_str(json_data).unwrap();
    
    // Convertir les Strings en chars pour simplifier la gestion
    let char_grid: Vec<Vec<char>> = grid.into_iter()
    .map(|row| row.into_iter().map(|cell| cell.chars().next().unwrap()).collect())
    .collect();

    commands.insert_resource(
        MazeResource {
            grid: char_grid.clone(),
            height: char_grid.len(),
            width: char_grid[0].len(),
        }
    );
}
