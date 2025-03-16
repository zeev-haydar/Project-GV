mod components;
mod resources;
mod spawns;
mod systems;

use crate::resources::camera::CameraState;
use crate::resources::game::{GameState, WorldAttribute};
use crate::resources::*;
use crate::spawns::ground::spawn_ground;
use crate::spawns::light::spawn_light;
use crate::spawns::player::spawn_player;
use crate::spawns::ui::{setup_debug_ui, setup_game_ui};
use crate::systems::camera::*;
use crate::systems::player::*;
use crate::systems::ui::update_player_info_system;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use std::time::Duration;
use bevy::window::{PresentMode, WindowMode};
use crate::spawns::structures::spawn_boxes;
use crate::spawns::wall::spawn_wall;
use log::info;

fn hello_world() {
    println!("hello world!");
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_attribute: Res<WorldAttribute>,
) {
    // spawn_camera(&mut commands);
    spawn_light(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials, &world_attribute);
    spawn_player(&mut commands, &mut meshes, &mut materials);
    spawn_wall(&mut commands, &mut meshes, &mut materials, &world_attribute);
    spawn_boxes(&mut commands, &mut meshes, &mut materials);
}

fn main() {
    env_logger::init();
    let all_systems = (
        player_position_info_system,
        camera_system,
        toggle_camera_mode_system,
        player_game_state_system,
        update_player_info_system,
    );

    let game_systems = (
            player_check_ground_system,
            keyboard_input_system,
        ).chain();

    App::new()
        .add_plugins((
            // DefaultPlugins.set(WindowPlugin {
            //     primary_window: Some(Window {
            //         mode: WindowMode::Fullscreen(MonitorSelection::Primary),
            //         present_mode: PresentMode::AutoVsync,
            //         ..default()
            //     }),
            //     ..default()
            // }),
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .insert_resource(DebugPrintTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        )))
        .insert_resource(ClearColor(Color::srgb_u8(127, 127, 127)))
        .insert_resource(WorldAttribute::default())
        .init_resource::<CameraState>()
        .init_resource::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(Startup, (setup_debug_ui, setup_game_ui).chain())
        .add_systems(Update, all_systems)
        .add_systems(Update, game_systems)
        .run();
}
