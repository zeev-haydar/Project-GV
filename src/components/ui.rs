use bevy::prelude::Component;

#[derive(Debug)]
pub enum Info {
    Position,
    FPS,
    Direction,
}

#[derive(Debug, Component)]
pub struct InfoText {
    pub info: Info,
}

#[derive(Debug, Component)]
pub struct FpsText;

/// Component to mark the UI node that displays the inventory.
#[derive(Component)]
pub struct InventoryUI;

/// Marker for an inventory slot UI node with its slot index.
#[derive(Component)]
pub struct InventorySlot {
    pub slot: usize,
}

/// Marker for the item image inserted into an inventory slot.
#[derive(Component)]
pub struct InventorySlotImage;

#[derive(Component)]
pub struct WeaponSlot;

#[derive(Component)]
pub struct WeaponSlotImage;

#[derive(Component)]
pub struct WeaponDurabilityText;