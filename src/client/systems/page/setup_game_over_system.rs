use crate::client::components::page_ui::GameOverUI;
use bevy::prelude::*;

pub fn spawn_game_over_ui(mut commands: Commands) {
    println!("Je suis dans spawn_game_over_ui");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // center children
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::BLACK),
            ));
        });
}

pub fn cleanup_game_over_ui(mut commands: Commands, query: Query<Entity, With<GameOverUI>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
