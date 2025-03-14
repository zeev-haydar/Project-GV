use bevy::input::ButtonInput;
use bevy::log::debug;
use bevy::math::Vec3;
use bevy::prelude::{KeyCode, Query, Res, ResMut, Time, Transform, With};
use crate::components::player::{Movement, Player, PlayerControlled};
use crate::resources::DebugPrintTimer;

/**
Read the keyboard event
*/
pub fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movement), With<PlayerControlled>>,
) {
    for (mut transform, movement) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            debug!("W was pressed");
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            debug!("S was pressed");
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            debug!("A was pressed");
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            debug!("D was pressed");
            direction.z += 1.0;
        }

        // normalize the direction vector to prevent diagonal movement being faster
        if direction != Vec3::ZERO {
            direction = direction.normalize();
        }

        // Apply the movement
        transform.translation += direction * movement.speed * time.delta_secs();
    }
}

pub fn player_position_info_system(
    mut query: Query<(&Transform, &Player), With<PlayerControlled>>,
    time: Res<Time>,
    mut timer: ResMut<DebugPrintTimer>,
) {
    // Only run the logic if the timer finished
    if timer.0.tick(time.delta()).just_finished() {
        for (transform, player) in query.iter() {
            print!("Player name: {}", player.name);
            print!(", ");
            println!("Position: {:?}", transform.translation);
        }
    }
}