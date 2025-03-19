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
}

impl IncreaseSpeedBundle {
    pub fn new(mesh: Mesh3d, material: MeshMaterial3d<StandardMaterial>, collider: Collider) -> Self {
        Self {
            mesh: mesh,
            material: material,
            item: Item {
                name: String::from("Increase Speed"),
                description: String::from("Add increasing speed for amount of time"),
                type_: ItemType::Passive,
                effect: ItemEffect::IncreaseSpeed { amount: (10.0), duration: (10.0) },
            },
            collider: collider,
            sensor: Sensor::default(),
        }
    }
}
