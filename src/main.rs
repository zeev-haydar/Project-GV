mod components;
mod resources;
mod spawns;
mod systems;
mod states;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};
use crate::states::game::game_plugin;
use crate::states::GameState;
use crate::states::menu::menu_plugin;


fn maximize_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.set_maximized(true);
    }

}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3d::default());;
}

fn main() {

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::Windowed,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            // DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, maximize_window)
        .add_plugins((menu_plugin, game_plugin))
        .run();
}
