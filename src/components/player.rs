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
pub struct Movement {
    pub(crate) speed: f32,
}

/// Define an item component. You can extend it with additional fields.
#[derive(Component, Debug, Clone)]
pub struct Item {
    pub name: String,
}

/// Define the Inventory component with exactly 5 slots.
#[derive(Component, Debug)]
pub struct Inventory {
    /// Each slot can hold an item or be empty.
    pub slots: [Option<Item>; 5],
}

impl Inventory {
    /// Creates a new, empty inventory.
    pub fn new() -> Self {
        Self {
            slots: [None, None, None, None, None],
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
}




