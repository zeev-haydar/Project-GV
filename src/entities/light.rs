use bevy::prelude::*;

pub fn spawn_light(commands:&mut Commands) {
    commands.spawn((
        PointLight {
            intensity: 0.0,
            range: 500.0,
            color: Color::WHITE,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(5.0, 5.0, 0.0),
    ));
}
