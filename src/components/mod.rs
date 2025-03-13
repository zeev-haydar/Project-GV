use bevy::math::Vec3;
use bevy::prelude::{Component};

pub mod person;
pub mod player;

#[derive(Component)]
pub struct Id(i64);