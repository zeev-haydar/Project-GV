use bevy::prelude::*;

pub fn spawn_light(commands:&mut Commands) {
    let x_rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4);
    let z_rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);

    // Combine the rotations (order matters)
    let combined_rotation = z_rotation * x_rotation;
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0, // Intensity of the sunlight
            shadows_enabled: true,
            ..Default::default()
        },
        Transform {
            // Rotate the light to simulate a sun angle. Here it's angled 45° downward.
            rotation: combined_rotation,
            ..Default::default()
        },
    ));
}
