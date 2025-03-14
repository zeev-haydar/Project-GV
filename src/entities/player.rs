use bevy::prelude::*;
use bevy::color::palettes::css::RED;
use crate::components::player::*;
use crate::resources::*;

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player {
            name: "Frieren".to_string(),
        },
        Transform::from_xyz(0.0, 1.5, 0.0),
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
