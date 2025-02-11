use bevy::prelude::*;

pub fn display_fps(
    time: Res<Time>,
    mut query: Query<&mut Text2d>,
) {
    let fps = 1.0 / time.delta_secs();
    println!("FPS: {:.2}", fps);
    if let Ok(mut _text) = query.get_single_mut() {
        // text.sections[1].value = format!("{:.1}", fps);
    }
}

pub fn setup_fps_counter(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    
    commands.spawn((
        Text2d::new("FPS: 0.0"),
        text_font.clone(),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(Color::WHITE),
    ));
}