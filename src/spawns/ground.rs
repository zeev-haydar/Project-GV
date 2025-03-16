use crate::resources::game::WorldAttribute;
use bevy::color::palettes::css::LIME;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::world::Ground;

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    attributes: &Res<WorldAttribute>,
) {
    let width = attributes.width;
    let height = attributes.height;
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::default()),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(width / 2.0, height / 2.0)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: attributes.ground_color,
            ..Default::default()
        })),
        Collider::cuboid(width / 2., 2., height / 2.),
        RigidBody::Fixed,
        Ground // a component to tag this entity as ground
    ));
}
