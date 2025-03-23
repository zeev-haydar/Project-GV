use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::world::{AabbCollider, Ground, EntityName, NotGround, Structure};

pub fn spawn_box(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: &Vec3,
    color: &Color,
    size: &Vec3,
) {
    let mesh = meshes.add(Cuboid::from_size(*size));
    let material = materials.add(StandardMaterial {
       base_color: *color,
        ..default()
    });
    let y_offset:f32 = 0.0;

    commands.spawn(
        (
            Transform::from_xyz(position.x, position.y+y_offset, position.z),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            EntityName::new("Box"),
            Visibility::default(),
            Structure,
            Friction {
                    coefficient: 0.0,
                    combine_rule: CoefficientCombineRule::Min,
                },
            Restitution {
                    coefficient: 0.0,
                        ..default()
                },
            Ground,
            Collider::cuboid(size.x/2., size.y/2., size.z/2.),
            RigidBody::Fixed,
            AabbCollider {
                    half_extents: Vec3::new(size.x / 2.0, size.y/2., size.z / 2.0),
                }
            ));
    // ).with_children(|parent| {
    //     // spawn collider for box as Structure
    //     let ground_layer_thickness: f32 = 0.05;
    //
    //     // This is the "Not Ground" part of the box
    //     parent.spawn(
    //         (
    //             Transform::from_xyz(0.0, -ground_layer_thickness, 0.0),
    //             NotGround,
    //             Friction {
    //                 coefficient: 0.0,
    //                 combine_rule: CoefficientCombineRule::Min,
    //             },
    //             Restitution {
    //                 coefficient: 0.0, // No bounciness
    //                 combine_rule: CoefficientCombineRule::Min,
    //             },
    //             Collider::cuboid(size.x/2.,(size.y-ground_layer_thickness)/2., size.z/2.)
    //         )
    //     );
    //
    //     // This is the "Ground" part of the box
    //     parent.spawn(
    //         (
    //             Transform::from_xyz(0.0, (size.y-ground_layer_thickness)/2.,0.0),
    //             Ground,
    //             Friction {
    //                 coefficient: 0.0,
    //                 combine_rule: CoefficientCombineRule::Min,
    //             },
    //             Restitution {
    //                 coefficient: 0.0, // No bounciness
    //                 combine_rule: CoefficientCombineRule::Min,
    //             },
    //             Collider::cuboid(size.x/2.,ground_layer_thickness/2., size.z/2.),
    //             AabbCollider {
    //                 half_extents: Vec3::new(size.x/2.,ground_layer_thickness/2., size.z/2.),
    //             }
    //             ),
    //
    //
    //     );
    // });
}

pub fn spawn_boxes(
    commands:&mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let position1 = Vec3::new(10.5, 1.5, -10.5);
    let position2 = Vec3::new(20.0, 1.5, 20.0);
    let size: Vec3 = Vec3::new(6.0, 2.0, 6.0);
    let color: Color = Color::WHITE;
    spawn_box(commands, meshes, materials, &position1, &color,&size);
    spawn_box(commands, meshes, materials, &position2, &color,&size);
}