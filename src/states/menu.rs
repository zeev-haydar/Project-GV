use bevy::audio::CpalSample;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use crate::states::*;

/// All state available at Menu
#[derive(Clone, Copy, Eq, Default, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled
}

#[derive(Component)]
struct OnMainMenuScreen;

/// All actions that can be done by menu buttons
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit
}

const NUMBER_OF_BUTTONS: i32 = 2;

/// Component to mark currently selected option
#[derive(Component)]
struct SelectedOption;

pub fn menu_plugin(app: &mut App) {
    app.
        init_state::<MenuState::Main>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(Update, (menu_actions).run_if(in_state(GameState::Menu)));
}

fn menu_setup(mut commands: &mut Commands, mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
    commands.spawn(Camera3d::default());
}

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font_handler = asset_server.load("fonts/OpenSans-Bold.ttf");

    let button_font = TextFont {
        font_size: 32.0,
        font: font_handler,
        ..default()
    };
    // spawn main menu UI Node
    commands
        .spawn(
            (
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                OnMainMenuScreen,
                BackgroundColor(MAGENTA.into())
            )
        )
        .with_children(|parent| {
            // This is the button container
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        width: Val::Percent(50.0),
                        ..default()
                    }
                    ))
                .with_children(|parent| {
                    // Spawn Play Button
                    spawn_button(
                        parent,
                        "Play".to_string(),
                        button_font.clone(),
                        MenuButtonAction::Play,
                    );

                    // Spawn Quit Button
                    spawn_button(
                        parent,
                        "Quit".to_string(),
                        button_font.clone(),
                        MenuButtonAction::Quit,
                    );
                });
        });
}

fn spawn_button(mut builder: &mut ChildBuilder, text: String, font: TextFont, menu_button_action: MenuButtonAction) {
    builder
        .spawn(
            (Node {
                    width: Val::Percent(100.0),
                    max_height: Val::Percent(100f32/NUMBER_OF_BUTTONS.to_float_sample()),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Button,
                BackgroundColor(WHITE.into()),
                menu_button_action,
        )).with_children(|parent| {
        parent.spawn((
                Text::new(text),
                font,
                TextColor(BLACK.into()),
                Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                }
            ));
    });
}

fn menu_actions(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match action {
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}