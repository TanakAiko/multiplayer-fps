use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationResource {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph_handle: Handle<AnimationGraph>, 
}

impl Default for AnimationResource {
    fn default() -> Self {
        Self {
            animations: Vec::new(),
            graph_handle: Default::default(),
        }
    }
}    

#[derive(Resource)]
pub struct AnimationState {
    pub current_animation: usize,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            current_animation: 0
        }
    }
}