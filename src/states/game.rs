use std::time::Duration;
use bevy::prelude::*;
use crate::resources::camera::CameraState;
use crate::resources::DebugPrintTimer;
use crate::resources::game::WorldAttribute;
use crate::spawns::ground::spawn_ground;
use crate::spawns::item::spawn_items;
use crate::spawns::light::spawn_light;
use crate::spawns::player::spawn_player;
use crate::spawns::structures::spawn_boxes;
use crate::spawns::ui::{setup_debug_ui, setup_game_ui};
use crate::spawns::wall::spawn_wall;
use crate::states::*;
use crate::systems::camera::{camera_system, toggle_camera_mode_system};
use crate::systems::player::{change_selected_item_system, check_item_intersections, check_weapon_durability_system, melee_system, player_movement_system, speed_boost_system, threw_item_system, update_jump_state_system, use_item_system};
use crate::systems::ui::{update_durability_text_system, update_inventory_ui_system, update_player_info_system};
use crate::systems::window::{hide_cursor, toggle_cursor};


pub fn game_setup(
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


pub fn game_plugin(
    app: &mut App,
) {
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

    app
        .insert_resource(DebugPrintTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        )))
        .insert_resource(ClearColor(Color::srgb_u8(127, 127, 127)))
        .insert_resource(WorldAttribute::default())
        .init_resource::<CameraState>()
        .add_systems(OnEnter(GameState::Game), (game_setup, hide_cursor, spawn_items))
        .add_systems(OnEnter(GameState::Game), (setup_debug_ui, setup_game_ui).chain())
        .add_systems(Update, all_systems.run_if(in_state(GameState::Game)))
        .add_systems(Update, game_systems.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

#[derive(Component)]
struct OnGameScreen;


