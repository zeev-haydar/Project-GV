use bevy::prelude::*;
use bevy::color::palettes::css::RED;
use bevy_rapier3d::prelude::*;
use crate::components::camera::{CameraSensitivity, PlayerCamera};
use crate::components::player::{Player, JumpAbility, Movement};

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player {
            name: "Frieren".to_string(),
        },
        Transform::from_xyz(0.0, 5., 0.0),
        Visibility::default(),
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: RED.into(),
            ..Default::default()
        })),
        Movement {
            speed: 10.0,
        },
        CameraSensitivity::default(),
        Collider::cuboid(0.5, 0.5, 0.5),
        RigidBody::Dynamic,
        Restitution::coefficient(0.1),
        Velocity {
            linvel: Vec3::ZERO,
            angvel: Vec3::ZERO,
        },
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        ExternalImpulse::default(),
        JumpAbility::default(),
    )).insert(ActiveEvents::COLLISION_EVENTS)
        .with_children(|parent| {
        parent.spawn((
                PlayerCamera,

                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..Default::default()
                }),
        ));
    });
}
