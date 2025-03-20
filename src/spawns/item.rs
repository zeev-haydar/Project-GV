use bevy::color::palettes::tailwind::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::components::player::{Item, ItemEffect, ItemType, SpeedBoost};

#[derive(Bundle, Clone)]
pub struct IncreaseSpeedBundle {
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    item: Item,
    collider: Collider,
    sensor: Sensor,
    collision_group: CollisionGroups,
    active_collision_types: ActiveCollisionTypes
}

impl IncreaseSpeedBundle {
    pub fn new(mesh: Mesh3d, material: MeshMaterial3d<StandardMaterial>, collider: Collider) -> Self {
        Self {
            mesh,
            material,
            item: Item {
                name: String::from("Increase Speed"),
                description: String::from("Add increasing speed for amount of time"),
                type_: ItemType::Passive,
                effect: ItemEffect::IncreaseSpeed { amount: (10.0), duration: (10.0) },
            },
            collider,
            sensor: Sensor::default(),
            collision_group: CollisionGroups::new(
                Group::GROUP_2,
                Group::GROUP_1
            ),
            active_collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_STATIC
        }
    }
}

pub fn spawn_items(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_positions = [
        Vec3::new(-15.0,  0.75,15_f32),
        Vec3::new(15.0,  0.75,-15_f32),
    ];

    let mesh= meshes.add(Sphere::new(0.5));
    let material = materials.add(StandardMaterial {
        base_color: Color::from(YELLOW_300),
        ..default()
    });

    // spawn the items
    commands.spawn(
        (IncreaseSpeedBundle::new(
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Collider::ball(0.5)
        ),
        Transform::from_translation(spawn_positions[0])
        )
    );

    commands.spawn(
        (IncreaseSpeedBundle::new(
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Collider::ball(0.5)
        ),
         Transform::from_translation(spawn_positions[1])
        )
    );
}
