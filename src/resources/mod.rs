use bevy::prelude::*;

#[derive(Resource)]
pub struct DebugPrintTimer(pub(crate) Timer);