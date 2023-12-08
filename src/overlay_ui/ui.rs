use bevy::prelude::*;

use crate::{statey::ExploreState, DisplayUiState};

use super::UiOverlay;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

const INTRO_TEXT: &str = "This game is in alpha, This game is in alpha, This game is in alpha, This game is in alpha, This game is in alpha, This game is in alpha, This game is in alpha, This game is in alpha, ";

#[derive(Component)]
pub struct ButtonGo;

#[derive(Component)]
pub struct ButtonBack;

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut explore_state: ResMut<NextState<ExploreState>>,
) {
    explore_state.set(ExploreState::Paused);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    grid_template_columns: vec![GridTrack::auto()],
                    grid_template_rows: vec![
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                    ],
                    ..default()
                },

                background_color: BackgroundColor(Color::Rgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                    alpha: 0.15,
                }),
                ..default()
            },
            UiOverlay,
        ))
        .with_children(|builder| {
            // Choose a name
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(25.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Percent(15.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), INTRO_TEXT, 30.0, Color::GREEN);
                });
            // name input

            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(25.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                })
                .with_children(|builder| {
                    spawn_nested_text_bundle(builder, font.clone(), INTRO_TEXT, 30.0, Color::RED);
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(50.0),
                        height: Val::Percent(50.0),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        grid_template_rows: vec![GridTrack::auto()],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..default()
                })
                .with_children(|builder| {
                    setup_go_button(builder, font.clone(), "Buy\n8192 sats".to_string());
                    setup_back_button(builder, font.clone(), "Back".to_string());
                });
            // button
        });
}

fn setup_go_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Center,
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonGo,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.1, 0.1, 0.1),
                        },
                    ));
                });
        });
}

fn setup_back_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Center,
                padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonBack,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.1, 0.1, 0.1),
                        },
                    ));
                });
        });
}

#[allow(clippy::type_complexity)]
pub fn go_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonGo>),
    >,
    //mut text_query: Query<&mut Text>,
    //mut game_state: ResMut<NextState<DisplayUiState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonBack>),
    >,
    //mut text_query: Query<&mut Text>,
    mut overlay_state: ResMut<NextState<DisplayUiState>>,
    mut explore_state: ResMut<NextState<ExploreState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                overlay_state.set(DisplayUiState::Off);
                explore_state.set(ExploreState::On);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn spawn_nested_text_bundle(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    color: Color,
) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size,
            color,
        },
    ));
}
