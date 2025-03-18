use std::f32::consts::FRAC_PI_2;
use bevy::input::mouse::{AccumulatedMouseMotion, MouseButtonInput, MouseMotion, MouseWheel};
use bevy::prelude::*;
use crate::components::camera::{CameraSensitivity, PlayerCamera};
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
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let delta = accumulated_mouse_motion.delta;
    if delta == Vec2::ZERO {
        return;
    }

    // Process horizontal rotation for the player (yaw)
    let delta_yaw = -delta.x * player_query.single().1.x; // using the sensitivity.x
    let (mut player_transform, camera_sensitivity) = match player_query.get_single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    let (mut current_yaw, _, _) = player_transform.rotation.to_euler(EulerRot::YXZ);
    current_yaw += delta_yaw;
    // Only apply yaw rotation to the player (ignore pitch and roll)
    player_transform.rotation = Quat::from_euler(EulerRot::YXZ, current_yaw, 0.0, 0.0);

    // Process vertical rotation for the camera (pitch)
    let delta_pitch = -delta.y * camera_sensitivity.y; // using the sensitivity.y
    let mut camera_transform = match camera_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Get current pitch from the camera transform (in Euler angles, YXZ ordering: yaw, pitch, roll)
    let (_, mut current_pitch, _) = camera_transform.rotation.to_euler(EulerRot::YXZ);
    current_pitch += delta_pitch;
    // Clamp the pitch to prevent flipping
    const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
    current_pitch = current_pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
    // Apply only the pitch rotation to the camera (no yaw or roll)
    camera_transform.rotation = Quat::from_euler(EulerRot::YXZ, 0.0, current_pitch, 0.0);
}