use crate::components::{player::*, world::*};
// use crate::resources::game::GameState;
// use crate::resources::DebugPrintTimer;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/**
Read the keyboard event
*/
pub fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut Transform,
            &Movement,
            &mut Velocity,
            &mut ExternalImpulse,
            Option<&mut JumpAbility>,
        ),
        With<Player>,
    >,
) {
    let Ok((mut transform, movement, mut v, mut impulse, jump_ability_opt)) =
        player_query.get_single_mut()
    else {
        return;
    };
    let mut input_dir = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input_dir.z -= 1.0
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input_dir.z += 1.0
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input_dir.x -= 1.0
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input_dir.x += 1.0
    }

    // Check if it is jumping
    if let Some(mut jump_ability) = jump_ability_opt {
        if keyboard_input.just_pressed(KeyCode::Space) && !jump_ability.is_jumping {
            v.linvel.y = 0.0;
            impulse.impulse = Vec3::new(0.0, 10.0, 0.0);
            jump_ability.is_jumping = true;
        }
        // else if !jump_ability.is_jumping {
        //         v.linvel.y = 0.0;
        // }
    }

    // if it is not jumping

    // Check if the direction input is applied
    if input_dir != Vec3::ZERO {
        input_dir = input_dir.normalize();

        // Get movement direction based on transform yaw
        let rotation = transform.rotation;
        let mut move_direction = rotation * input_dir;

        // Prevent Y-Axis movement
        move_direction.y = 0.0;

        // Update velocity based on input
        let dx = move_direction.x * movement.speed * time.delta_secs() * 50.0;
        let dz = move_direction.z * movement.speed * time.delta_secs() * 50.0;
        v.linvel = Vec3::new(dx, v.linvel.y, dz);
    } else {
        // No input detected; set velocity to zero
        v.linvel = Vec3::new(0.0, v.linvel.y, 0.0);
    }
}

// pub fn player_position_info_system(
//     mut query: Query<(&Transform, &Player), With<Player>>,
//     time: Res<Time>,
//     mut timer: ResMut<DebugPrintTimer>,
// ) {
//     return;
//     // Only run the logic if the timer finished
//     if timer.0.tick(time.delta()).just_finished() {
//         for (transform, player) in query.iter() {
//             print!("Player name: {}", player.name);
//             print!(", ");
//             println!("Position: {:?}", transform.translation.to_string());
//         }
//     }
// }

pub fn player_check_ground_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut JumpAbility, &mut Velocity), With<Player>>,
    ground_query: Query<Entity, With<Ground>>,
) {
    // Attempt to get the player entity
    let Some((player_entity, mut jump_ability, mut velocity)) = player_query.get_single_mut().ok()
    else {
        return; // If no player found, exit early
    };

    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
            println!("Collision detected: {:?} and {:?}", entity1, entity2);

            // Check if either entity is the player
            if *entity1 == player_entity || *entity2 == player_entity {
                // Check if the other entity is any ground entity
                for ground_entity in ground_query.iter() {
                    if *entity1 == ground_entity || *entity2 == ground_entity {
                        println!("Player landed on the ground");
                        jump_ability.is_jumping = false;
                        velocity.linvel.y = 0.0; // auto disable any y velocity no matter what
                        break; // No need to check further ground entities
                    }
                }
            }
        }

        if let CollisionEvent::Stopped(entity1, entity2, _) = collision_event {
            if *entity1 == player_entity || *entity2 == player_entity {
                for ground_entity in ground_query.iter() {
                    if *entity1 == ground_entity || *entity2 == ground_entity {
                        println!("Player left the ground");
                        jump_ability.is_jumping = true;
                        break;
                    }
                }
            }
        }
    }
}

// pub fn player_check_ground_system_with_raycast(
//     rapier_context: ReadDefaultRapierContext,
//     // This query will be used to check if the hit entity is tagged as "Ground"
//     ground_query: Query<&Ground>,
//     mut player_query: Query<(&Transform, &mut JumpAbility), With<Player>>,
// ) {
//     let ray_length: Real = 2.0;
//
//     for (player_transform, mut jump_ability) in player_query.iter_mut() {
//         let ray_pos = player_transform.translation;
//         let ray_dir = Vec3::new(0.0, -1.0, 0.0);
//
//         // Cast the ray
//         if let Some((hit_entity, _toi )) = rapier_context.cast_ray(
//             ray_pos,
//             ray_dir,
//             ray_length,
//             true,
//             QueryFilter::default(),
//         ) {
//             // println!("ini keprint");
//             let hit_point = ray_pos + ray_dir;
//             // println!("Entity {:?} hit at point {}", hit_entity, hit_point);
//             if ground_query.get(hit_entity).is_ok() {
//                 println!("ini keprint");
//                 jump_ability.is_jumping = false;
//             }
//         }
//     }
// }

// pub fn update_jump_state_system(
//     mut player_query: Query<(&Transform, &mut JumpAbility), With<Player>>,
//     ground_query: Query<(&Transform, &AabbCollider)>, // All ground objects must have AabbCollider
// ) {
//     // Adjust the ray length to be slightly longer than half the player's height.
//     let ray_length = 1.334;
//     // We're casting the ray downward.
//     let ray_direction = Vec3::NEG_Y; // (0, -1, 0)

//     for (player_transform, mut jump_ability) in player_query.iter_mut() {
//         // let ray_origin = player_transform.translation;
//         let mut grounded = false;

//         // Check against each ground entity.
//         for (ground_transform, aabb) in ground_query.iter() {
//             // The AABB is defined by the ground entity's translation (center) and its half extents.
//             if let Some(t) = multi_ray_intersect_bottom_aabb(
//                 ground_transform.translation,
//                 aabb.half_extents,
//                 0.1,
//             ) {
//                 println!("Ray hit at distance: {}", t);
//                 // If intersection occurs within ray_length, consider the player grounded.
//                 if t >= 0.0 && t <= ray_length {
//                     grounded = true;
//                     break;
//                 }
//             }
//         }
//         jump_ability.is_jumping = !grounded;
//     }
// }

/// System that updates the player's jump state by checking if the player is "grounded"
/// via raycasts cast from the bottom of the player's collider box.
pub fn update_jump_state_system(
    mut player_query: Query<(&Transform, &mut JumpAbility), With<Player>>,
    ground_query: Query<(&Transform, &AabbCollider)>, // All ground objects must have an AabbCollider
) {
    // Adjust the ray length to be slightly longer than half the player's height.
    let ray_length = 0.5;
    // Force the ray direction to be downward.
    let ray_direction = Vec3::NEG_Y;

    // In this example we assume a fixed box for the player.
    // In a real scenario, this might be part of a collider component.
    let player_half_extents = Vec3::new(0.5, 0.5, 0.5);

    for (player_transform, mut jump_ability) in player_query.iter_mut() {
        let mut grounded = false;

        // Check each ground entity.
        for (ground_transform, ground_collider) in ground_query.iter() {
            if let Some(t) = multi_ray_intersect_from_box(
                player_transform.translation, // Player's box center
                player_half_extents,          // Player's half extents
                ground_transform.translation, // Ground's box center
                ground_collider.half_extents, // Ground's half extents
                0.1,                          // Step size for sampling
                ray_direction,
            ) {
                // println!("Ray hit at distance: {}", t);
                if t >= 0.0 && t <= ray_length {
                    grounded = true;
                    break;
                }
            }
        }
        jump_ability.is_jumping = !grounded;
    }
}
