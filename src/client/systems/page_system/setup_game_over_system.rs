use crate::client::components::page_ui::GameOverUI;
use bevy::prelude::*;

pub fn spawn_game_over_ui(mut commands: Commands) {
    commands
        .spawn((BackgroundColor(Color::BLACK), GameOverUI, Node::default()));
        // .with_children(|parent| {
        //     parent.spawn((TextSpan::default(), TextColor(Color::WHITE)));
        // });
}

pub fn cleanup_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
