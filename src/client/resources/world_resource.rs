use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct MazeResource {
    pub grid: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
}

impl Default for MazeResource {
    fn default() -> Self {
        Self {
            grid: Vec::new(),
            height: 0,
            width: 0,
        }
    }
}