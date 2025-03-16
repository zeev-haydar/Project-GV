use crate::components::player::*;
use crate::resources::game::GameState;
use crate::resources::DebugPrintTimer;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::world::Ground;
use log::info;

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
            Option<&mut JumpAbility>
        ),
        With<Player>,
    >,
) {
    let Ok((
               mut transform, movement,
               mut v, mut impulse,
               jump_ability_opt)) = player_query.get_single_mut() else {
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
            impulse.impulse = Vec3::new(0.0, 5.0, 0.0);
            jump_ability.is_jumping = true; // Mark as jumping
        }

        if !jump_ability.is_jumping {
            v.linvel.y = 0.0;
        }
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

pub fn player_position_info_system(
    mut query: Query<(&Transform, &Player), With<Player>>,
    time: Res<Time>,
    mut timer: ResMut<DebugPrintTimer>,
) {
    return;
    // Only run the logic if the timer finished
    if timer.0.tick(time.delta()).just_finished() {
        for (transform, player) in query.iter() {
            print!("Player name: {}", player.name);
            print!(", ");
            println!("Position: {:?}", transform.translation.to_string());
        }
    }
}

pub fn player_game_state_system(
    player_query: Query<&Transform, With<Player>>,
    mut game_state: ResMut<GameState>,
) {
    let Ok(player_transform) = player_query.get_single() else {
        return;
    };

    // game_state.position = player_transform.translation;
}

pub fn player_check_ground_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut JumpAbility, &mut Velocity), With<Player>>,
    ground_query: Query<Entity, With<Ground>>
) {
    // Attempt to get the player entity
    let Some((player_entity, mut jump_ability, mut velocity)) = player_query.get_single_mut().ok() else {
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
