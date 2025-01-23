use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

pub fn load_maze_from_json(path: &str) -> Vec<Vec<char>> {
    let json_data = include_str!("maze.json");
    let grid: Vec<Vec<String>> = serde_json::from_str(json_data).unwrap();

    // Convertir les Strings en chars pour simplifier la gestion
    grid.into_iter()
        .map(|row| row.into_iter().map(|cell| cell.chars().next().unwrap()).collect())
        .collect()
}
