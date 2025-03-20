/* player.rs */
use bevy::prelude::*;

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

impl Default for PlayerStats {
    fn default() -> Self {
        Self { health: 100.0, speed: 15.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Passive,
    Active,
}

/// The effect of an item when used
#[derive(Clone)]
pub enum ItemEffect {
    IncreaseSpeed { amount: f32, duration: f32 }, // Increase movement speed
    Heal(f32),         // Restore health
    Throw(Vec3),       // Throw in a direction
    MeleeAttack(u32),  // Melee attack with durability
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

/// Define the Inventory component with exactly 5 slots.
#[derive(Component)]
pub struct Inventory {
    /// Each slot can hold an item or be empty.
    pub slots: [Option<Item>; 5],
    pub current_selected_item: usize,
}

impl Inventory {
    /// Creates a new, empty inventory.
    pub fn new() -> Self {
        Self {
            slots: [None, None, None, None, None],
            current_selected_item: 0
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
    pub fn use_item(&mut self, player: &mut PlayerStats, commands: &mut Commands, entity: Entity) {
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
                ItemEffect::Throw(direction) => {
                    println!("Item thrown in direction {:?}", direction);
                    // Implement logic to spawn a thrown object
                }
                ItemEffect::MeleeAttack(durability) => {
                    println!("Attacked with melee weapon, durability left: {}", durability);
                    if durability > 1 {
                        self.slots[self.current_selected_item] =
                            Some(Item { effect: ItemEffect::MeleeAttack(durability - 1), ..item });
                    }
                }
            }
        }
    }

}




