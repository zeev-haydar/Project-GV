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
use crate::systems::ui::{update_durability_text_system, update_inventory_ui_system, update_player_info_system};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use std::time::Duration;
use bevy::window::{PresentMode, PrimaryWindow, WindowMode};
use crate::spawns::item::spawn_items;
use crate::spawns::structures::spawn_boxes;
use crate::spawns::wall::spawn_wall;
use crate::systems::window::{hide_cursor, toggle_cursor};


fn maximize_window(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = windows.get_single_mut() {
        window.set_maximized(true);
    }

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
    let all_systems = (
        camera_system,
        toggle_camera_mode_system,
        update_player_info_system,
        toggle_cursor,
        update_inventory_ui_system,
        update_durability_text_system,
        threw_item_system
    );

    let game_systems = (
            update_jump_state_system,
            player_movement_system,
            melee_system,
            check_weapon_durability_system,
            check_item_intersections,
            change_selected_item_system,
            use_item_system,
            speed_boost_system
        ).chain();

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
        .insert_resource(DebugPrintTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        )))
        .insert_resource(ClearColor(Color::srgb_u8(127, 127, 127)))
        .insert_resource(WorldAttribute::default())
        .init_resource::<CameraState>()
        .init_resource::<GameState>()
        .add_systems(Startup, (maximize_window, setup, hide_cursor, spawn_items))
        .add_systems(Startup, (setup_debug_ui, setup_game_ui).chain())
        .add_systems(Update, all_systems)
        .add_systems(Update, game_systems)
        .run();
}
