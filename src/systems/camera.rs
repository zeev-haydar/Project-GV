use std::f32::consts::FRAC_PI_2;
use bevy::input::mouse::{AccumulatedMouseMotion, MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use crate::components::camera::CameraSensitivity;
use crate::components::player::*;
use crate::resources::camera::*;

// Toggle camera mode with V key
pub fn toggle_camera_mode_system(
    mut camera_state: ResMut<CameraState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,

) {
    if keyboard_input.just_pressed(KeyCode::KeyV) {
        camera_state.mode = match camera_state.mode {
            CameraMode::FirstPerson => CameraMode::ThirdPerson,
            CameraMode::ThirdPerson => CameraMode::FirstPerson,
        };
    }
}

// Unified camera system
pub fn camera_system(
    camera_state: ResMut<CameraState>,
    mut player_query: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let Ok((mut transform, camera_sensitivity)) = player_query.get_single_mut() else {
        return;
    };

    let delta = accumulated_mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }


}
