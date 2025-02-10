use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct AnimationComponent {
    pub graph_handle: Handle<AnimationGraph>,  // represente la structure des animations (graph)
    pub index: AnimationNodeIndex, // identifiant (nombre) d'un nœud particulier dans le graphe d'animation auquel on souhaite accéder (ou que l'on souhaite jouer).
}