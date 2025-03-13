mod components;
mod resources;
mod systems;

use std::time::Duration;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use crate::components::player::{keyboard_input_system, player_position_info_system, Movement, Player, PlayerControlled};
use crate::resources::DebugPrintTimer;

fn hello_world() {
    println!("hello world!");
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn camera
    commands.spawn(
        (
            Camera3d::default(),
            Transform::from_xyz(10.0, 12.0, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
        )
    );

    commands.spawn(
        (
            PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..Default::default()
            },
            Transform::from_xyz(4.0, 8.0, 4.0),
            )
    );

    // spawn a player entity
    commands.spawn((
            Player {
                name: "Frieren".to_string(),
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::default(),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                    ..Default::default()
            })),
            Movement {
                speed: 10.0,
            },
            PlayerControlled,

        ));
    }


fn main() {
    let all_systems = (
        keyboard_input_system,
        player_position_info_system
    );

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DebugPrintTimer(Timer::new(Duration::from_millis(100), TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, all_systems)
        .run();
}
