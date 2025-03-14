use bevy::prelude::*;
use crate::components::player::Player;

pub fn camera_follow_system(
    mut camera_query: Query<(&mut Transform), With<Camera3d>>,
    player_query: Query<(&GlobalTransform), With<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for mut camera_transform in camera_query.iter_mut() {
            let offset = Vec3::new(0.0, 35f32, -5.0);

            // Set camera position relative to player
            camera_transform.translation = player_transform.translation() + offset;

            // Make camera look at player
            camera_transform.look_at(player_transform.translation(), Vec3::Y);
        }
    }
}