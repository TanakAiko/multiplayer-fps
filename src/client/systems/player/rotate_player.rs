use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::client::components::{camera_component::CameraSensitivity, player_component::Player};

pub fn rotate_player(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    // Extraction du joueur Single
    let Ok((mut transform, camera_sensitivity)) = query.get_single_mut() else {
        return
    };

    let delta = mouse_motion.delta;

    if delta != Vec2::ZERO {

        // Ici c'est le calcul de la rotation suivant les axes.
        // Et enfin la camera sensitivity ajuste cela
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        // Limiter la rotation verticale du joueur;
        const PITCH_LIMIT:f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch +  delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);

    }


}
