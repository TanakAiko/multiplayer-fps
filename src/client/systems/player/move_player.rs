use bevy::prelude::*;

use crate::client::{
    components::player_component::{Player, PlayerStep},
    resources::player_resource::PlayerResource,
    systems::player::step::playsoundsprint,
};

pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut res_player: ResMut<PlayerResource>,
    music_controller: Query<&AudioSink, With<PlayerStep>>,
) {
    const SPEED: f32 = 0.06;
    let mut direction = Vec3::ZERO;

    for mut transform in query.iter_mut() {
        let player_forward_horizontal =
            Vec3::new(transform.forward().x, 0.0, transform.forward().z).normalize();

        let player_right_horizontal =
            Vec3::new(transform.right().x, 0.0, transform.right().z).normalize();

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        let move_direction = (player_forward_horizontal * direction.z
            + player_right_horizontal * direction.x)
            .normalize_or_zero()
            * SPEED;
        transform.translation += move_direction;
        res_player.position = transform.translation.clone();

        let is_moving = direction != Vec3::ZERO;
        playsoundsprint(&music_controller, is_moving);
    }
}
