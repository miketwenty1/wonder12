use bevy::prelude::*;

use super::{
    components::{Capitalizable, KeyBoard, KeyBoardButton, KeyType, KeyboardNode},
    styles::NORMAL_BUTTON,
};

const NUMBER_SET: &str = "1234567890";
const FUNCTION_SET: &str = "<^ ";
const LETTER_SET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn setup_keyboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    placement_query: Query<Entity, With<KeyboardNode>>,
) {
    info!("keyboard setup!");
    for ent in placement_query.iter() {
        // let mut parent_node = commands.entity(ent);
        // parent_node.commands().spawn(bundle)
        let key_chars = ["@1234567890<", "qwertyuiop", "^asdfghjkl", "zxcvbnm ."];
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let mut keyboardcmds = commands.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(99.0),
                    height: Val::Percent(99.0),
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
                        GridTrack::flex(1.0),
                    ],
                    ..default()
                },
                background_color: BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
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
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[0]);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[1]);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[2]);
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(0.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    spawn_keyboard_row(builder, font.clone(), key_chars[3]);
                });
        });

        keyboardcmds.set_parent(ent);
    }
}

fn spawn_keyboard_row(builder: &mut ChildBuilder, font: Handle<Font>, row_keys: &str) {
    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(99.0),
                height: Val::Percent(99.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect {
                    left: Val::Percent(1.),
                    right: Val::Percent(1.),
                    top: Val::Percent(0.1),
                    bottom: Val::Percent(0.1),
                },
                justify_content: JustifyContent::SpaceEvenly,
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
            for key in row_keys.chars() {
                keyboard_button(builder, font.clone(), key);
            }
        });
}

fn keyboard_button(builder: &mut ChildBuilder, font: Handle<Font>, key: char) {
    let key_type: KeyType;

    if LETTER_SET.contains(key) {
        key_type = KeyType::Letter;
    } else if NUMBER_SET.contains(key) {
        key_type = KeyType::Number;
    } else if FUNCTION_SET.contains(key) {
        key_type = KeyType::Function;
    } else {
        key_type = KeyType::Letter; //console_log!("a key is not defined as a type")
    }

    builder
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::horizontal(Val::Px(3.0)),
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
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    KeyBoardButton(key),
                    key_type.clone(),
                ))
                .with_children(|parent| {
                    let ent_text = parent
                        .spawn(TextBundle::from_section(
                            key.to_string(),
                            TextStyle {
                                font,
                                font_size: 32.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ))
                        .id();

                    if key_type == KeyType::Letter {
                        parent.add_command(bevy::ecs::system::Insert {
                            entity: ent_text,
                            bundle: Capitalizable,
                        });
                    }
                });
        });
}
