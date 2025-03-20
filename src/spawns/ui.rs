use crate::components::ui::*;
use bevy::color::palettes::css::ORANGE;
use bevy::prelude::*;

pub fn setup_debug_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI Text
    let font = asset_server.load("fonts/OpenSans.ttf");

    let text_font = TextFont {
        font: font.clone(),
        font_size: 24.0,
        ..default()
    };

    // commands.spawn(Camera2d);

    commands
        .spawn((
            Node {
                width: Val::Percent(25.),
                height: Val::Percent(25.),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                top: Val::Px(5.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0., 0., 0., 0.5)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Player Info"),
                text_font.clone(),
                TextColor(ORANGE.into()),
                InfoText {
                    info: Info::Position,
                },
            ));
            parent.spawn((
                Text::new("FPS"),
                text_font.clone(),
                TextColor(ORANGE.into()),
                InfoText { info: Info::FPS },
            ));
        });
}

pub fn setup_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn crosshair
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|parent| {
            // This is our crosshair
            parent
                .spawn(Node {
                    width: Val::Px(20.0),
                    height: Val::Px(20.0),
                    position_type: PositionType::Relative,
                    ..default()
                })
                .with_children(|crosshair| {
                    // Horizontal line
                    crosshair.spawn((
                        Node {
                            width: Val::Px(20.),
                            height: Val::Px(2.),
                            position_type: PositionType::Absolute,
                            top: Val::Px(9.),
                            left: Val::Px(0.),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE.into()),
                    ));

                    // Vertical line
                    crosshair.spawn((
                        Node {
                            width: Val::Px(2.),
                            height: Val::Px(20.),
                            position_type: PositionType::Absolute,
                            top: Val::Px(0.),
                            left: Val::Px(9.),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE.into()),
                    ));
                });
        });

    // spawn inventory slot
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.),
            right: Val::Px(10.),
            display: Display::Flex,
            flex_direction: FlexDirection::RowReverse,
            ..default()
        })
        .with_children(|parent| {
            for slot in 0..5 {
                parent.spawn((
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(50.),
                        height: Val::Px(50.),
                        border: UiRect::all(Val::Px(2.5)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgba(0.2, 0.2, 0.2, 0.5).into()),
                    BorderColor(Color::WHITE.into()),
                    InventorySlot {slot}
                ));
            }
        });
}
