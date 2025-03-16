use std::collections::VecDeque;
use std::time::Duration;
use bevy::diagnostic::{Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::info;
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
    player_query: Query<(&Transform), With<Player>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for (mut text, info_text) in info_text_query.iter_mut() {
        match info_text.info {
            Info::Position => {
                if let Ok(transform) = player_query.get_single() {
                    update_position_text(&mut text, &transform)
                }
            },
            Info::FPS => update_fps_text(&mut text, &diagnostics),
        }
    }
}

fn update_position_text(text: &mut Text, transform: &Transform) {
    set_text(text, format!(
        "Position = {:.2} {:.2} {:.2}",
        transform.translation.x,
        transform.translation.y,
        transform.translation.z
    ));
}

/// Updates text for FPS
fn update_fps_text(text: &mut Text, diagnostics: &DiagnosticsStore) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        set_text(text, format!("FPS: {:.0}", fps));
    }
}

/// Generic function to update text
fn set_text(text: &mut Text, value: String) {
    text.0 = value;
}