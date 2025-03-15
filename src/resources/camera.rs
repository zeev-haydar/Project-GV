// Camera mode resource
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct CameraState {
    pub mode: CameraMode,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target_offset: Vec3,
}

#[derive(PartialEq, Default)]
pub enum CameraMode {
    #[default]
    ThirdPerson,
    FirstPerson,
}
