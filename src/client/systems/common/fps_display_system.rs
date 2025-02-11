use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

pub fn display_fps(time: Res<Time>, mut query: Query<&mut TextSpan, With<FpsText>>) {
    let fps = 1.0 / time.delta_secs();
    for mut text in &mut query {
        **text = format!("{:.1}", fps);
    }
}

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 20.0,
        ..default()
    };

    commands
        .spawn((
            Text2d::new("FPS: "),
            text_font.clone(),
            Transform::from_xyz(-580.0, 320.0, 0.0),
            TextColor(Color::WHITE),
        ))
        .with_child((TextSpan::default(), text_font.clone(), FpsText));
}
