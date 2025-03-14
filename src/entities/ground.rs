use bevy::prelude::*;
use bevy::color::palettes::css::LIME;

pub fn spawn_ground(
    mut commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::default()),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(
            materials.add(StandardMaterial {
                base_color: LIME.into(),
                ..Default::default()
            }),
        ),
    ));
}
