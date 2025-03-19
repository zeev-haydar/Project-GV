use bevy::prelude::Component;

#[derive(Debug)]
pub enum Info {
    Position,
    FPS,
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