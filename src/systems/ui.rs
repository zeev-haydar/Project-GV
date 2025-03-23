use crate::components::player::{Direction, Inventory, Player};
use crate::components::ui::{
    FpsText, Info, InfoText, InventorySlot, InventorySlotImage, WeaponDurabilityText, WeaponSlot,
    WeaponSlotImage,
};
use bevy::color::palettes::tailwind::{GRAY_400, GRAY_500, GRAY_700, ORANGE_500, RED_400};
use bevy::diagnostic::{Diagnostics, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::image::*;
use bevy::prelude::*;
use bevy::render::camera::RenderTarget::Image;

// pub fn update_debug_info_system(
//     mut fps_history: Local<VecDeque<f64>>,
//     mut time_history: Local<VecDeque<Duration>>,
//     time: Res<Time>,
//     diagnostics: Res<DiagnosticsStore>,
//     mut text_query: Query<(&mut Text2d), With<InfoText>>,
//     mut writer:TextUiWriter,
// ) {
//     let Ok(text) = text_query.get_single_mut() else {
//         return;
//     };
//
//     //
// }

pub fn update_player_info_system(
    mut info_text_query: Query<(&mut Text, &InfoText), With<InfoText>>,
    player_query: Query<(&Transform, &Direction), With<Player>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    for (mut text, info_text) in info_text_query.iter_mut() {
        match info_text.info {
            Info::Position => {
                if let Ok((transform, _)) = player_query.get_single() {
                    update_position_text(&mut text, &transform)
                }
            }
            Info::FPS => update_fps_text(&mut text, &diagnostics),
            Info::Direction => {
                if let Ok((_, direction)) = player_query.get_single() {
                    update_direction_text(&mut text, &direction.direction)
                }
            }
        }
    }
}

fn update_position_text(text: &mut Text, transform: &Transform) {
    set_text(
        text,
        format!(
            "Position = {:.2} {:.2} {:.2}",
            transform.translation.x, transform.translation.y, transform.translation.z
        ),
    );
}

fn update_direction_text(text: &mut Text, direction: &Vec3) {
    set_text(
        text,
        format!(
            "Direction = {:.2} {:.2} {:.2}",
            direction.x, direction.y, direction.z
        ),
    )
}

/// Updates text for FPS
fn update_fps_text(text: &mut Text, diagnostics: &DiagnosticsStore) {
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        set_text(text, format!("FPS: {:.0}", fps));
    }
}

/// Generic function to update text
fn set_text(text: &mut Text, value: String) {
    text.0 = value;
}

pub fn update_inventory_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Inventory, With<Player>>,
    mut item_slot_query: Query<(
        Entity,
        &mut BorderColor,
        &mut InventorySlot,
        Option<&Children>,
    )>,
    mut weapon_slot_query: Query<(Entity, &mut WeaponSlot, Option<&Children>)>,
    mut inventory_slot_image_query: Query<Entity, With<InventorySlotImage>>,
    mut weapon_slot_image_query: Query<Entity, With<WeaponSlotImage>>,
) {
    if let Ok(inventory) = player_query.get_single() {
        for (slot_entity, mut border, slot_marker, item_slot_children_opt) in
            item_slot_query.iter_mut()
        {
            if slot_marker.slot == inventory.current_selected_item {
                border.0 = Color::from(RED_400);
            } else {
                border.0 = Color::WHITE;
            }

            if let Some(Some(item)) = &inventory.slots.get(slot_marker.slot) {
                let asset_path = match item.name.as_str() {
                    "Nasi Bungkus" => "textures/nasi_bungkus.png",
                    _ => "textures/default_item.png",
                };

                let image_handle = asset_server.load(asset_path);

                // Check if this slot already has an image child
                let mut has_image = false;
                if let Some(children) = item_slot_children_opt {
                    for &child in children.iter() {
                        if inventory_slot_image_query.get(child).is_ok() {
                            has_image = true;
                            break;
                        }
                    }
                }

                // If not, then spawn one
                if !has_image {
                    commands.entity(slot_entity).with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            })
                            .insert(ImageNode::new(image_handle))
                            .insert(InventorySlotImage);
                    });
                }
            } else {
                // The slot is empty, remove any child image if exists
                if let Some(children) = item_slot_children_opt {
                    for &child in children.iter() {
                        if inventory_slot_image_query.get(child).is_ok() {
                            commands.entity(child).despawn_recursive();
                        }
                    }
                }
            }
        }
        for (weapon_slot_entity, weapon_slot, weapon_slot_opt_children) in
            weapon_slot_query.iter_mut()
        {
            // update ui for weapon slot
            if let Some(my_weapon) = &inventory.weapon {
                let asset_path = match my_weapon.name.as_str() {
                    "Golok" => "textures/golok.png",
                    _ => "textures/default_item.png",
                };

                let asset_font = asset_server.load("fonts/OpenSans.ttf");

                let image_handle = asset_server.load(asset_path);

                // Check if the weapon slot has a child
                let mut has_image = false;
                if let Some(children) = weapon_slot_opt_children {
                    for &child in children.iter() {
                        if weapon_slot_image_query.get(child).is_ok() {
                            has_image = true;
                            break;
                        }
                    }
                }

                if !has_image {
                    commands.entity(weapon_slot_entity).with_children(|parent| {
                        parent
                            .spawn(Node {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                ..default()
                            })
                            .insert(ImageNode::new(image_handle))
                            .insert(WeaponSlotImage)
                            .with_children(|slot| {
                                slot.spawn((
                                    Node {
                                        left: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        position_type: PositionType::Absolute,
                                        ..default()
                                    },
                                    Text::new(my_weapon.durability.to_string()),
                                    TextColor(Color::from(ORANGE_500)),
                                    TextFont {
                                        font: asset_font,
                                        font_size: 12.0,
                                        ..default()
                                    },
                                    WeaponDurabilityText,
                                ));
                            });
                    });
                }
            } else {
                if let Some(children) = weapon_slot_opt_children {
                    for &child in children.iter() {
                        if weapon_slot_image_query.get(child).is_ok() {
                            commands.entity(child).despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}

pub fn update_durability_text_system(
    inventory_query: Query<&Inventory, With<Player>>,
    mut text_query: Query<&mut Text, With<WeaponDurabilityText>>,
) {
    if let Ok(inventory) = inventory_query.get_single() {
        if let Some(weapon) = &inventory.weapon {
            for mut text in text_query.iter_mut() {
                text.0 = weapon.durability.to_string();
            }
        }
    }
}
