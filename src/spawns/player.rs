use crate::components::camera::{CameraSensitivity, PlayerCamera};
use crate::components::player::{Direction, Inventory, JumpAbility, Player, PlayerStats};
use crate::components::world::EntityName;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    name: EntityName,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    movement: PlayerStats,
    camera_sensitivity: CameraSensitivity,
    collider: Collider,
    rigid_body: RigidBody,
    restitution: Restitution,
    velocity: Velocity,
    friction: Friction,
    gravity_scale: GravityScale,
    locked_axes: LockedAxes,
    jump_ability: JumpAbility,
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PlayerBundle {
            player: Player,
            name: EntityName::new("Frieren"),
            global_transform: GlobalTransform::default(),
            transform: Transform::from_xyz(0.0, 5., 0.0),
            visibility: Visibility::default(),
            mesh: Mesh3d(meshes.add(Cuboid::default())),
            material: MeshMaterial3d(materials.add(StandardMaterial {
                base_color: RED.into(),
                ..Default::default()
            })),
            movement: PlayerStats { speed: 15.0, ..Default::default() },
            camera_sensitivity: CameraSensitivity::default(),
            collider: Collider::cuboid(0.5, 0.5, 0.5),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::coefficient(0.0),
            velocity: Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO,
            },
            friction: Friction::coefficient(0.0),
            gravity_scale: GravityScale(2.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            jump_ability: JumpAbility::default(),
        })
        .insert(ExternalImpulse::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::default())
        .insert(Inventory::new())
        .insert(CollisionGroups::new(
            Group::GROUP_1,
            Group::GROUP_2 | Group::GROUP_3,
        ))
        .insert(Direction::default())
        .with_children(|parent| {
            parent.spawn((
                Transform::from_xyz(0., 1.5, 0.),
                GlobalTransform::default(),
                PlayerCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 80_f32.to_radians(),
                    ..Default::default()
                }),
            ));
        });
}

pub fn spawn_random_item(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {

}
