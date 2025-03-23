/* player.rs */
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::world::ThrewObject;

#[derive(Component)]
pub struct Player;


#[derive(Debug, Component)]
pub struct JumpAbility {
    pub is_jumping: bool,
}

impl Default for JumpAbility {
    fn default() -> Self {
        Self {
            is_jumping: true,
        }
    }
}

#[derive(Component)]
pub struct PlayerStats {
    pub health: f32,
    pub(crate) speed: f32,
}

#[derive(Component)]
pub struct Direction {
    pub(crate) direction: Vec3
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO
        }
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { health: 100.0, speed: 15.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Passive,
    Active,
    Weapon,
}

/// The effect of an item when used
#[derive(Clone)]
pub enum ItemEffect {
    IncreaseSpeed { amount: f32, duration: f32 }, // Increase movement speed
    Heal(f32),         // Restore health
    Throw(Handle<Mesh>, Handle<StandardMaterial>),       // Throw in a direction
    WeaponItem(Weapon),  // Melee attack with durability
}

#[derive(Component)]
pub struct SpeedBoost {
    pub amount: f32,
    pub timer: Timer,
}
/// Define an item component. You can extend it with additional fields.
#[derive(Component, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub type_:ItemType,
    pub effect: ItemEffect,
}

#[derive(Component, Clone)]
pub struct Weapon {
    pub name: String,
    pub description: String,
    pub throwable: bool,
    pub durability: u16
}

impl Weapon {
    pub fn decrement_durability(&mut self) {
        self.durability = self.durability.saturating_sub(1);
    }
}

/// Define the Inventory component with exactly 5 slots.
#[derive(Component)]
pub struct Inventory {
    /// Each slot can hold an item or be empty.
    pub slots: [Option<Item>; 5],
    pub weapon: Option<Weapon>,
    pub current_selected_item: usize,
}

impl Inventory {
    /// Creates a new, empty inventory.
    pub fn new() -> Self {
        Self {
            slots: [None, None, None, None, None],
            current_selected_item: 0,
            weapon: None
        }
    }

    /// Attempts to add an item to the first available slot.
    /// Returns Ok(()) if successful, or Err(item) if the inventory is full.
    pub fn add_item(&mut self, item: Item) -> Result<(), Item> {
        for slot in &mut self.slots {
            if slot.is_none() {
                *slot = Some(item);
                return Ok(());
            }
        }
        Err(item)
    }

    /// Removes an item from a specific slot by index.
    /// Returns the removed item, or None if the slot was empty or index is invalid.
    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index < self.slots.len() {
            self.slots[index].take()
        } else {
            None
        }
    }

    /// Use the selected item and remove them
    pub fn use_item(&mut self,
                    player: &mut PlayerStats,
                    transform: Option<Mut<Transform>>,
                    direction: Option<&Direction>,
                    commands: &mut Commands,
                    entity: Entity,
                    time: &Res<Time>) {
        if let Some(item) = self.slots[self.current_selected_item].take() {
            match item.effect {
                ItemEffect::IncreaseSpeed { amount, duration } => {

                    player.speed += amount;
                    println!("Speed increased by {}", amount);

                    // Add a SpeedBoost component to track the duration
                    commands.entity(entity).insert(SpeedBoost {
                        amount,
                        timer: Timer::from_seconds(duration, TimerMode::Once),
                    });
                }
                ItemEffect::Heal(amount) => {
                    player.health += amount;
                    println!("Healed by {}", amount);
                }
                ItemEffect::Throw(
                                  handle_mesh,
                                  handle_material
                ) => {
                    let Some(direction) = direction else {
                        return;
                    };
                    let Some(origin) = transform else {
                        return;
                    };
                    println!("Item thrown in direction {:?}", direction.direction);
                    // spawn thrown object
                    let speed = 30f32;
                    commands.spawn(
                        (
                                Velocity {
                                    linvel: speed * direction.direction,
                                        ..Default::default()
                                },
                                CollisionGroups::new(
                                    Group::GROUP_3,
                                    Group::GROUP_1 | Group::GROUP_2,
                                ),
                                ActiveEvents::COLLISION_EVENTS,
                                ThrewObject {
                                    spawn_time: time.elapsed_secs()
                                },
                                RigidBody::Dynamic,
                                GravityScale(1.0),
                                Transform::from_translation(origin.translation + Vec3::from((direction.direction.x, 1., direction.direction.z))),
                                Collider::ball(0.5),
                                Mesh3d(handle_mesh),
                                MeshMaterial3d(handle_material),
                                Sensor
                            )
                    );

                }
                ItemEffect::WeaponItem(weapon) => {
                    // insert this bruh to weapon slot
                    self.weapon = Some(weapon);
                }
            }
        }
    }

}




