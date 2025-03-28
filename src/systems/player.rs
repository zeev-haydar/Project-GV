use crate::components::{player::*, world::*};
// use crate::resources::game::GameState;
// use crate::resources::DebugPrintTimer;
use bevy::input::{ButtonInput, ButtonState};
use bevy::input::mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel};
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::text::cosmic_text::Scroll;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::CollisionEventFlags;

/**
Read the keyboard event
*/
pub fn player_movement_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<
        (
            &mut Transform,
            &PlayerStats,
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
        let dx = move_direction.x * movement.speed;
        let dz = move_direction.z * movement.speed;
        v.linvel = Vec3::new(dx, v.linvel.y, dz);
    } else {
        // No input detected; set velocity to zero
        v.linvel = Vec3::new(0.0, v.linvel.y, 0.0);
    }
}

pub fn use_item_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Inventory, &mut PlayerStats, Option<&mut Transform>, Option<&Direction>), With<Player>>,
) {
    for (entity, mut inventory, mut player_stats, transform, direction) in player_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::KeyE) {
            inventory.use_item(&mut player_stats, transform, direction, &mut commands, entity, &time);
        }
    }
}

pub fn change_selected_item_system(
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    for mut inventory in inventory_query.iter_mut() {
        for event in scroll_events.read() {
            match event.unit {
                MouseScrollUnit::Line => {
                    if event.y > 0.0 {
                        // Scroll up: Move to the next item
                        inventory.current_selected_item =
                            (inventory.current_selected_item + 1) % inventory.slots.len();
                    } else if event.y < 0.0 {
                        // Scroll down: Move to the previous item (wrapping around)
                        inventory.current_selected_item = (inventory.current_selected_item + inventory.slots.len() - 1) % inventory.slots.len();
                    }
                    // println!(
                    //     "Selected item index: {}",
                    //     inventory.current_selected_item
                    // );
                }
                _ => {}
            }
        }
    }
}

pub fn melee_system (
    mut mouse_input: EventReader<MouseButtonInput>,
    mut inventory_query: Query<&mut Inventory, With<Player>>,
) {
    for event in mouse_input.read() {
        match (event.button, event.state)  {
            (MouseButton::Left, ButtonState::Pressed) => {
                if let Ok(mut inventory) = inventory_query.get_single_mut() {
                    if let Some(ref mut weapon) = inventory.weapon {
                        weapon.decrement_durability();
                        println!("Weapon durability: {}", weapon.durability);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn check_weapon_durability_system(
    mut inventory_query: Query<&mut Inventory, With<Player>>,
) {
    let Ok(mut inventory) = inventory_query.get_single_mut() else {
        return;
    };

    // check if the durability of the weapon is zero
    if let Some(weapon) = &inventory.weapon {
        if weapon.durability == 0 {
            // Set the weapon to None
            inventory.weapon = None;
            println!("Weapon broken and removed from inventory!");
        }
    }
}


pub fn speed_boost_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut PlayerStats, &mut SpeedBoost)>,
) {
    for (entity, mut player_stats, mut speed_boost) in player_query.iter_mut() {
        speed_boost.timer.tick(time.delta());

        if speed_boost.timer.finished() {
            player_stats.speed -= speed_boost.amount;
            println!("Speed boost expired. Speed reset to {}", player_stats.speed);
            commands.entity(entity).remove::<SpeedBoost>();
        }
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
            // if let Some(t) = multi_ray_intersect_from_box(
            //     player_transform.translation,
            //     player_half_extents,
            //     ground_transform.translation,
            //     ground_collider.half_extents,
            //     0.025,
            //     ray_direction,
            // ) {
            //     // println!("Ray hit at distance: {}", t);
            //     if t >= 0.0 && t <= ray_length {
            //         grounded = true;
            //         break;
            //     }
            // }
            if let Some(t) = slab_ray_intersect_aabb(
                player_transform.translation,
                player_half_extents,
                ground_transform.translation,
                ground_collider.half_extents,
                Vec3::NEG_Y, // Downward ray
                0.1f32
            ) {
                // println!("Hit detected at distance: {}", t);
                if t >= 0.0 && t <= ray_length {
                    grounded = true;
                    break;
                }
            }
        }
        jump_ability.is_jumping = !grounded;
    }
}

pub fn check_item_intersections(
    mut commands: Commands,
    mut collider_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &Transform, &Player, &EntityName, &mut PlayerStats, &mut Inventory)>,
    mut item_query: Query<(Entity, &Item, &Transform)>,
) {
    for event in collider_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, flag) if *flag == CollisionEventFlags::SENSOR => {
                // Try player as entity1 and item as entity2
                if let Some((player_name, mut inventory, item_entity, item)) =
                    try_get_player_and_item(*entity1, *entity2, &mut player_query, &mut item_query)
                {
                    handle_item_pickup(&mut commands, player_name, &mut inventory, item_entity, item);
                }
                // Try player as entity2 and item as entity1
                else if let Some((player_name, mut inventory, item_entity, item)) =
                    try_get_player_and_item(*entity2, *entity1, &mut player_query, &mut item_query)
                {
                    handle_item_pickup(&mut commands, player_name, &mut inventory, item_entity, item);
                }
            }
            _ => {}
        }
    }
}

pub fn threw_item_system(
    mut commands: Commands,
    time: Res<Time>,
    mut threw_item_query: Query<(Entity, &ThrewObject)>,
    ground_query: Query<(Entity, &Ground)>,
    mut player_query: Query<(Entity, &mut PlayerStats), With<Player>>,
    wall_query: Query<(Entity, &Wall)>,
    mut collider_events: EventReader<CollisionEvent>,
) {
    let current_time = time.elapsed_secs(); // Get the current game time

    for (entity, threw_object) in threw_item_query.iter_mut() {
        // Check if the object has been alive for more than 5 seconds
        if current_time - threw_object.spawn_time > 5.0 {
            println!("Threw Object {} despawned due to timeout", entity);
            commands.entity(entity).despawn_recursive();
            continue;
        }
    }

    for event in collider_events.read() {
        match event {
            CollisionEvent::Started(entity1, entity2, flag)
                if *flag == CollisionEventFlags::SENSOR  => {

                        if let Ok((threw_object_entity, threw_object)) = threw_item_query.get(*entity1) {
                            // if let Ok((player_entity, player_stats)) = player_query.get_mut(*entity2) {
                            //     println!("Threw Object {} hit player {}", threw_object_entity, player_entity);
                            //
                            //     // destroy the object
                            //     commands.entity(threw_object_entity).despawn_recursive();
                            // }
                            if let Ok((ground_entity, _)) = ground_query.get(*entity2) {
                                println!("Threw Object {} hit ground {}", threw_object_entity, ground_entity);
                                commands.entity(threw_object_entity).despawn_recursive();
                            }
                            else if let Ok((wall_entity, _)) =  wall_query.get(*entity2) {
                                println!("Threw Object {} hit wall {}", threw_object_entity, wall_entity);
                                commands.entity(threw_object_entity).despawn_recursive();
                            }
                    }
            }
            _ => {}
        }
    }
}

// HELPER FUNCTION

/// Helper function to check if a player and item are involved in a collision
fn try_get_player_and_item<'a>(
    player_entity: Entity,
    item_entity: Entity,
    mut player_query: &'a mut Query<(Entity, &Transform, &Player, &EntityName, &mut PlayerStats, &mut Inventory)>,
    mut item_query: &'a mut Query<(Entity, &Item, &Transform)>,
) -> Option<(&'a EntityName, Mut<'a, Inventory>, Entity, &'a Item)> {
    if let Ok((_, _, _, player_name, _, mut inventory)) = player_query.get_mut(player_entity) {
        if let Ok((item_entity, item, _)) = item_query.get_mut(item_entity) {
            return Some((player_name, inventory, item_entity, item));
        }
    }
    None
}

/// Helper function to handle item pickup logic
fn handle_item_pickup(
    commands: &mut Commands,
    player_name: &EntityName,
    inventory: &mut Mut<Inventory>,
    item_entity: Entity,
    item: &Item,
) {
    println!("Player {:?} collided with item '{}'", player_name, item.name);

    // Pick up the item
    match inventory.add_item(item.clone()) {
        Ok(()) => {
            println!("Item '{}' added to inventory.", item.name);

            // Remove item from the world
            commands.entity(item_entity).despawn_recursive();
        }
        Err(_) => {
            println!("Inventory is full!");
        }
    }
}

