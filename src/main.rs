mod components;
mod resources;
mod systems;
mod entities;

use std::time::Duration;
use bevy::prelude::*;
use crate::entities::camera::spawn_camera;
use crate::entities::ground::spawn_ground;
use crate::entities::light::spawn_light;
use crate::entities::player::spawn_player;
use crate::resources::*;
use crate::systems::camera::camera_follow_system;
use crate::systems::player::*;

fn hello_world() {
    println!("hello world!");
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_camera(&mut commands);
    spawn_light(&mut commands);
    spawn_ground(&mut commands, &mut meshes, &mut materials);
    spawn_player(&mut commands, &mut meshes, &mut materials);
    }


fn main() {
    let all_systems = (
        keyboard_input_system,
        player_position_info_system,
        camera_follow_system
    );

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DebugPrintTimer(Timer::new(Duration::from_millis(500), TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, all_systems)
        .run();
}
