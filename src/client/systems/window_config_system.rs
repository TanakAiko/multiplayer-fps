use bevy::{prelude::*, window::PresentMode};

pub fn config_window(
    mut query_window: Single<&mut Window>,
) {
    let query_window_clone = query_window.clone();
    query_window.title = "Maze Game".to_string();
    query_window.cursor_options.visible = false;
    // query_window.cursor_options.grab_mode = CursorGrabMode::Locked; // Lock the cursor
    // Center cursor on window
    query_window.set_cursor_position(Some(Vec2::new(query_window_clone.width() as f32 / 2.0, query_window_clone.height() as f32 / 2.0)));
    query_window.decorations = true;
    query_window.present_mode = PresentMode::AutoVsync;

}