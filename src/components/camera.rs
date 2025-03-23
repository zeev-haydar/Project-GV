use bevy::prelude::*;

#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(
            Vec2::new(0.003, 0.002),
        )
    }
}

#[derive(Debug, Component)]
pub struct PlayerCamera;