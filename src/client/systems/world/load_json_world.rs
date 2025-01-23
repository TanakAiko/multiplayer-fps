use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MazeElement {
    x: f32,
    y: f32,
    z: f32,
    element_type: String,
    dimensions: Option<(f32, f32, f32)>,
}
pub fn load_maze_from_json(path: &str) -> Vec<MazeElement> {
    let contents = std::fs::read_to_string(path).expect("Impossible de lire le fichier");
    serde_json::from_str(&contents).expect("Erreur de parsing JSON")
}
