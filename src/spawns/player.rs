use std::f32::consts::FRAC_PI_2;
use crate::components::camera::{CameraSensitivity, PlayerCamera};
use crate::components::player::{JumpAbility, Movement, Player};
use bevy::color::palettes::css::RED;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{RapierContext};
use bevy_rapier3d::prelude::*;
use crate::resources::camera::CameraState;
pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Player::new("Frieren".to_string()),
            Transform::from_xyz(0.0, 5., 0.0),
            Visibility::default(),
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            })),
            Movement { speed: 20.0 },
            CameraSensitivity::default(),
            Collider::cuboid(0.5, 0.5, 0.5),
            RigidBody::Dynamic,
            Restitution::coefficient(0.0),
            Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO,
            },
            Friction::coefficient(0.0),
            GravityScale(2.0),
            LockedAxes::ROTATION_LOCKED,
            JumpAbility::default(),
        ))
        .insert(ExternalImpulse::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::default())
        .with_children(|parent| {
            parent.spawn((
                Transform::from_xyz(0., 1.5, 0.),
                PlayerCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..Default::default()
                }),
            ));
        });
}

