use std::collections::VecDeque;
use std::time::Duration;
use bevy::diagnostic::{Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use crate::components::player::Player;
use crate::components::text::{FpsText, Info, InfoText};
use crate::resources::game::GameState;

// pub fn update_debug_info_system(
//     mut fps_history: Local<VecDeque<f64>>,
//     mut time_history: Local<VecDeque<Duration>>,
//     time: Res<Time>,
//     diagnostics: Res<DiagnosticsStore>,
//     mut text_query: Query<(&mut Text2d), With<InfoText>>,
//     mut writer:TextUiWriter,
// ) {
//     let Ok(text) = text_query.get_single_mut() else {
//         return;
//     };
//
//     //
// }

pub fn update_player_info_system(
    mut info_text_query: Query<(&mut Text, &InfoText), With<InfoText>>,
    game_state: Res<GameState>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for (mut text, info_text) in info_text_query.iter_mut() {
        match info_text.info {
            Info::Position => update_position_text(&mut text, &game_state),
            Info::FPS => update_fps_text(&mut text, &diagnostics),
        }
    }
}

fn update_position_text(text: &mut Text, game_state: &GameState) {
    set_text(text, format!(
        "Position = {:.2} {:.2} {:.2}",
        game_state.position.x,
        game_state.position.y,
        game_state.position.z
    ));
}

/// Updates text for FPS
fn update_fps_text(text: &mut Text, diagnostics: &DiagnosticsStore) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        set_text(text, format!("FPS: {:.2}", fps));
    }
}

/// Generic function to update text
fn set_text(text: &mut Text, value: String) {
    text.0 = value;
}