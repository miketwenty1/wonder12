use bevy::prelude::*;

use crate::{
    eventy::{KeyboardSpawnEvent, NumberKeyboardSpawnEvent},
    resourcey::{ColorPalette, WinSize},
};

use super::{
    components::{Changeable, KeyBoard, KeyBoardButton, KeyboardNode, NumberKeyboardNode},
    KeyboardState,
};

// const NUMBER_SET: &str = "1234567890";
// const FUNCTION_SET: &str = "⌫⇧⬆ ";
// const LETTER_SET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn setup_keyboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<KeyboardNode>>,
    colors: Res<ColorPalette>,
    wsize: Res<WinSize>,
    mut event: EventReader<KeyboardSpawnEvent>,
    mut state: ResMut<NextState<KeyboardState>>,
) {
    for _e in event.read() {
        info!("keyboard setup!");
        state.set(KeyboardState::On);
        let font_size = if wsize.width < 420.0 { 22.0 } else { 32.0 };
        let padding_size = 1.0; //if wsize.width < 420.0 { 0.5 } else { 1.0 };
        let row_height = 40.0; //if wsize.height < 900.0 { 0.5 } else { 1.0 };
        let keyboard_row_justification = if wsize.width < 420.0 {
            JustifyContent::Center
        } else {
            JustifyContent::SpaceEvenly
        };
        for ent in placement_query.iter() {
            info!("is there a place for keyboard?");
            // let mut parent_node = commands.entity(ent);
            // parent_node.commands().spawn(bundle)
            let key_chars = [
                ("@1234567890-=⌫", "@!#$%[]&*()_+⌫"),
                ("qwertyuiop", "QWERTYUIOP"),
                ("⇧asdfghjkl:'", "⬆ASDFGHJKL;\""),
                ("zxcvbnm,.?", "ZXCVBNM<>/"),
                (" ", " "),
            ];

            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let mut keyboardcmds = commands.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        grid_template_columns: vec![GridTrack::flex(1.0)],
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        // gap: Size {
                        //     width: Val::Px(0.0),
                        //     height: Val::Px(0.0),
                        // },
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::flex(1.0),
                        ],
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    //z_index: ZIndex::Global(20),
                    ..default()
                },
                KeyBoard,
            ));

            keyboardcmds.with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },

                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[0],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[1],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[2],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[3],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[4],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
            });

            keyboardcmds.set_parent(ent);
        }
        info!("this runs no matter what for keyboard");
    }
}

pub fn setup_keyboard_numbers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<NumberKeyboardNode>>,
    colors: Res<ColorPalette>,
    wsize: Res<WinSize>,
    mut event: EventReader<NumberKeyboardSpawnEvent>,
    mut state: ResMut<NextState<KeyboardState>>,
) {
    for _e in event.read() {
        info!("keyboard setup!");
        state.set(KeyboardState::On);
        let font_size = if wsize.width < 420.0 { 22.0 } else { 32.0 };
        let padding_size = 1.0; //if wsize.width < 420.0 { 0.5 } else { 1.0 };
        let row_height = 40.0; //if wsize.height < 900.0 { 0.5 } else { 1.0 };
        let keyboard_row_justification = if wsize.width < 420.0 {
            JustifyContent::Center
        } else {
            JustifyContent::SpaceEvenly
        };
        for ent in placement_query.iter() {
            info!("is there a place for keyboard?");
            // let mut parent_node = commands.entity(ent);
            // parent_node.commands().spawn(bundle)
            let key_chars = [("789", "789"), ("456", "456"), ("123", "123"), ("0⌫", "0⌫")];

            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            let mut keyboardcmds = commands.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        grid_template_columns: vec![GridTrack::flex(1.0)],
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        // gap: Size {
                        //     width: Val::Px(0.0),
                        //     height: Val::Px(0.0),
                        // },
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::flex(1.0),
                        ],
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    //z_index: ZIndex::Global(20),
                    ..default()
                },
                KeyBoard,
            ));

            keyboardcmds.with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },

                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[0],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[1],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[2],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        spawn_keyboard_row(
                            builder,
                            font.clone(),
                            key_chars[3],
                            colors.button_color,
                            font_size,
                            padding_size,
                            keyboard_row_justification,
                        );
                    });
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            display: Display::Grid,
                            justify_items: JustifyItems::Center,
                            padding: UiRect::all(Val::Px(0.0)),
                            height: Val::Px(row_height),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        if key_chars.len() > 4 {
                            spawn_keyboard_row(
                                builder,
                                font.clone(),
                                key_chars[4],
                                colors.button_color,
                                font_size,
                                padding_size,
                                keyboard_row_justification,
                            );
                        }
                    });
            });

            keyboardcmds.set_parent(ent);
        }
        info!("this runs no matter what for keyboard");
    }
}

fn spawn_keyboard_row(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    row_keys: (&str, &str),
    button_color: Color,
    font_size: f32,
    padding_size: f32,
    keyboard_row_justification: JustifyContent,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(99.5),
                height: Val::Percent(99.5),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Px(1.),
                    right: Val::Px(1.),
                    top: Val::Px(1.),
                    bottom: Val::Px(1.),
                },
                justify_content: keyboard_row_justification,
                // gap: Size {
                //     width: Val::Px(0.0),
                //     height: Val::Px(0.0),
                // },
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            for (key, alt_key) in row_keys.0.chars().zip(row_keys.1.chars()) {
                keyboard_button(
                    builder,
                    font.clone(),
                    key,
                    alt_key,
                    button_color,
                    font_size,
                    padding_size,
                );
            }
            // for (key, alt_key) in row_keys.iter() {

            // }
        });
}

fn keyboard_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    key: char,
    alt_key: char,
    button_color: Color,
    font_size: f32,
    padding_size: f32,
) {
    // let key_type: KeyType;

    // if LETTER_SET.contains(key) {
    //     key_type = KeyType::Letter;
    // } else if NUMBER_SET.contains(key) {
    //     key_type = KeyType::Number;
    // } else if FUNCTION_SET.contains(key) {
    //     key_type = KeyType::Function;
    // } else {
    //     key_type = KeyType::Letter; //console_log!("a key is not defined as a type")
    // }

    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(Val::Px(padding_size)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            //let keyin = key_type.clone();
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            //size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    KeyBoardButton(key, alt_key),
                    //key_type.clone(),
                ))
                .with_children(|parent| {
                    let ent_text = parent
                        .spawn((
                            TextBundle::from_section(
                                key.to_string(),
                                TextStyle {
                                    font,
                                    font_size,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            KeyBoardButton(key, alt_key),
                        ))
                        .id();
                    parent.add_command(move |world: &mut World| {
                        world.entity_mut(ent_text).insert(Changeable);
                    });
                    // if key_type == KeyType::Letter {
                    //let a = parent;
                    // parent.add_command(|world: &mut World| {
                    //     world.entity_mut(ent_text).insert(Capitalizable)
                    // });

                    // parent.add_command(bevy::ecs::system::Insert {
                    //     entity: ent_text,
                    //     bundle: Capitalizable,
                    // });
                    // }
                });
        });
}
