/* player.rs */
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub name: String,
    camera_direction: Vec3,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            camera_direction: Vec3::Z,
        }
    }
}

#[derive(Debug, Component)]
pub struct JumpAbility {
    pub is_jumping: bool,
    pub is_grounded: bool,
}

impl Default for JumpAbility {
    fn default() -> Self {
        Self {
            is_jumping: true,
            is_grounded: false,
        }
    }
}

#[derive(Component)]
pub struct Movement {
    pub(crate) speed: f32,
}




