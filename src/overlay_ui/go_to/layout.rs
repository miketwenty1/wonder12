use bevy::prelude::*;

use crate::{
    componenty::{EditabledTextBox, UiInteractionBtn},
    consty::{DEFAULT_HEIGHT_INPUT_TEXT, DEFAULT_NO_PICK_COLOR},
    eventy::NumberKeyboardSpawnEvent,
    keyboard::{components::NumberKeyboardNode, resources::KeyboardData},
    resourcey::{ColorPalette, TargetType, WinSize},
    statey::ExploreSelectState,
};

use super::component::{GoToBackBtn, GoToGoBtn, GoToNode, GoToTextBoxButton, GoToTextBoxText};

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    win: Res<WinSize>,
    mut keyboard_event: EventWriter<NumberKeyboardSpawnEvent>,
    mut keyboard: ResMut<KeyboardData>,
    mut game_select_set_state: ResMut<NextState<ExploreSelectState>>,
) {
    game_select_set_state.set(ExploreSelectState::Off);
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let w_size = if win.width > 450.0 { 450.0 } else { win.width };
    // let h_size = if win.height > 300.0 {
    //     300.0
    // } else {
    //     win.height
    // };

    let font_size_text = if win.width < 420.0 { 18.0 } else { 24.0 };
    let edit_box_width = if win.width < 420.0 {
        (win.width / 2.0) - 1.0
    } else {
        210.0
    };
    let mut overlay_goto = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
            ..default()
        },
        GoToNode,
    ));

    overlay_goto.with_children(|builder| {
        let mut goto_node = builder.spawn(NodeBundle {
            style: Style {
                //display: Display::Grid,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Px(w_size),
                //max_height: Val::Px(h_size),
                ..default()
            },

            background_color: BackgroundColor(colors.node_color),
            ..default()
        });

        goto_node.with_children(|builder| {
            // // //
            // top row
            // // //
            let mut toprow = builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,

                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                background_color: BackgroundColor(colors.node_color),
                ..Default::default()
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            //padding: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn(TextBundle::from_section(
                            "Type in a Block Height to Travel",
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: colors.text_color,
                            },
                        ));
                    });
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder
                    .spawn(NodeBundle { ..default() })
                    .with_children(|innerbuilder| {
                        innerbuilder
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(30.0),
                                        height: Val::Px(30.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        align_content: AlignContent::Center,
                                        justify_items: JustifyItems::Center,
                                        ..default()
                                    },
                                    background_color: colors.red_color.into(),
                                    ..default()
                                },
                                UiInteractionBtn,
                                GoToBackBtn,
                            ))
                            .with_children(|ccbuilder| {
                                ccbuilder.spawn(TextBundle::from_section(
                                    "X",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 30.0,
                                        color: colors.text_color,
                                    },
                                ));
                            });
                    });
            });

            // Input Height text box
            height_input_box(
                builder,
                font.clone(),
                DEFAULT_HEIGHT_INPUT_TEXT.to_string(),
                colors.accent_color,
                colors.button_color,
                font_size_text,
                edit_box_width,
            );

            // go button
            setup_goto_go_button(builder, font.clone(), "Go".to_string(), colors.clone());

            //keyboard node
            builder.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(23.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        //padding: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    //background_color: BackgroundColor(Color::YELLOW),
                    ..default()
                },
                NumberKeyboardNode,
            ));
            keyboard_event.send(NumberKeyboardSpawnEvent);
            keyboard.target = TargetType::GoTo;
        });
    });
}

fn height_input_box(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
    font_size: f32,
    edit_box_width: f32,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                align_content: AlignContent::Center,
                //max_height: Val::Percent(50.0),
                //min_height: Val::Percent(50.0),
                min_width: Val::Px(edit_box_width),
                max_width: Val::Px(edit_box_width),
                min_height: Val::Px(36.0),
                max_height: Val::Px(36.0),
                //padding: UiRect::all(Val::Px(2.0)),
                //margin: UiRect::bottom(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            align_content: AlignContent::Center,
                            //margin: UiRect::all(Val::Px(1.0)),
                            flex_wrap: FlexWrap::Wrap,
                            min_width: Val::Px(edit_box_width - 2.0),
                            max_width: Val::Px(edit_box_width - 2.0),
                            min_height: Val::Px(34.0),
                            max_height: Val::Px(34.0),
                            //min_height: Val::Px(80.0),
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    UiInteractionBtn,
                    GoToTextBoxButton,
                    EditabledTextBox,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        TextBundle::from_section(
                            button_text.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size,
                                color: DEFAULT_NO_PICK_COLOR.into(),
                            },
                        ),
                        GoToTextBoxText,
                    ));
                });
        });
}

fn setup_goto_go_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    colors: ColorPalette,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Center,
                align_content: AlignContent::Start,
                //margin: UiRect::all(Val::Px(5.)),
                //padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            //background_color: BackgroundColor(Color::PURPLE), //colors.node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            min_width: Val::Px(80.0),
                            min_height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        border_color: BorderColor(colors.text_color),
                        background_color: colors.button_color.into(),
                        ..default()
                    },
                    UiInteractionBtn,
                    GoToGoBtn,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 28.0,
                            color: colors.text_color,
                        },
                    ));
                });
        });
}
