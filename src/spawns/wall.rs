use crate::resources::game::WorldAttribute;
use bevy::asset::Assets;
use bevy::math::{Vec2, Vec3};
use bevy::pbr::StandardMaterial;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::world::NotGround;

pub fn spawn_wall(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    world_attribute: &Res<WorldAttribute>,
) {
    let wall_material = materials.add(StandardMaterial {
        base_color: world_attribute.wall_color,
        ..default()
    });

    let wall_positions = [
        Vec3::new(
            0.0,
            world_attribute.wall_height / 2.0,
            world_attribute.height / 2.0,
        ),
        Vec3::new(
            0.0,
            world_attribute.wall_height / 2.0,
            world_attribute.height / -2.0,
        ),
        Vec3::new(
            world_attribute.width / 2.0,
            world_attribute.wall_height / 2.0,
            0_f32,
        ),
        Vec3::new(
            world_attribute.width / -2.0,
            world_attribute.wall_height / 2.0,
            0_f32,
        ),
    ];

    let wall_sizes = [
        Vec3::new(
            world_attribute.width,
            world_attribute.wall_height,
            world_attribute.wall_thickness,
        ), // Front & Back walls
        Vec3::new(
            world_attribute.wall_thickness,
            world_attribute.wall_height,
            world_attribute.height,
        ), // Left & Right walls
    ];

    for (i, &pos) in wall_positions.iter().enumerate() {
        let size = if i < 2 { wall_sizes[0] } else { wall_sizes[1] };

        commands.spawn((
            Transform::from_translation(pos),
            Mesh3d(meshes.add(Cuboid::from_size(size))),
            MeshMaterial3d(wall_material.clone()),
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            NotGround
        ));
    }
}
