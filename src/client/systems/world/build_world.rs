use bevy::prelude::*;

use super::load_json_world::load_maze_from_json;

// Définition d'un type de cellule plus riche
pub enum CellType {
    Wall,
    // Autres types d'éléments
}

// Représentation 2D du labyrinthe
// type MazeGrid = Vec<Vec<CellType>>;

pub fn create_maze_from_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let maze = load_maze_from_json("./src/client/maze.json")
}
