// Hides the cursor and locks it in the window at the start
use bevy::prelude::*;

pub fn hide_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
}

// Toggles cursor visibility when Escape is pressed
pub fn toggle_cursor(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    if keyboard_input.pressed(KeyCode::AltLeft) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None;
    } else {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    }
}