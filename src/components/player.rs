/* player.rs */
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
}

#[derive(Component)]
pub struct PlayerControlled;

#[derive(Component)]
pub struct Movement {
    pub(crate) speed: f32,
}




