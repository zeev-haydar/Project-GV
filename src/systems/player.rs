use crate::components::player::*;
use crate::resources::game::GameState;
use crate::resources::DebugPrintTimer;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::world::Ground;

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
    }

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

    game_state.position = player_transform.translation;
}

pub fn player_check_ground_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut JumpAbility), With<Player>>,
    ground_query: Query<Entity, With<Ground>>
) {
    let Ok((player_entity, mut jump_ability)) = player_query.get_single_mut() else {
        return;
    };
    let Ok(ground_entity) = ground_query.get_single() else {
        return;
    };
    // println!("ini keprint");
    for collision_event in collision_events.read() {
        println!("Received Collision Event: {:?}", collision_event);

        match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => {
                println!("entity1: {:?}, entity2: {:?}", entity1, entity2);


                println!("Player Entity: {:?}, Ground Entity: {:?}", player_entity, ground_entity);

                if (*entity1 == player_entity && *entity2 == ground_entity) || (*entity1 == ground_entity && *entity2 == player_entity) {
                    println!("player touch ground");
                    jump_ability.is_jumping = false;
                    // jump_ability.is_grounded = true;
                } else {
                    println!("not collide with ground");
                }

            }
            _ => {}
        }
    }
}
