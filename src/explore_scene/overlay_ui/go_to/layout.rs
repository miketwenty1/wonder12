use bevy::{prelude::*, text::FontSmoothing};

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
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            ..default()
        },
        GoToNode,
    ));

    overlay_goto.with_children(|builder| {
        let mut goto_node = builder.spawn((
            Node {
                //display: Display::Grid,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Px(w_size),
                border: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BorderColor(colors.node_color_lighter),
            BorderRadius::all(Val::Px(10.0)),
            BackgroundColor(colors.node_color),
        ));

        goto_node.with_children(|builder| {
            // // //
            // top row
            // // //
            let mut toprow = builder.spawn((
                Node {
                    width: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                BorderRadius::all(Val::Px(5.0)),
                BackgroundColor(colors.node_color),
            ));
            toprow.with_children(|builder| {
                builder.spawn(Node {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder.spawn(Node {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder.spawn(Node {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder
                    .spawn(Node {
                        margin: UiRect::vertical(Val::Px(8.0)),
                        ..default()
                    })
                    .with_children(|builder| {
                        builder.spawn((
                            Text::new("Type in a Block Height to Travel"),
                            TextFont {
                                font: font.clone(),
                                font_size: 20.0,
                                font_smoothing: FontSmoothing::AntiAliased,
                            },
                            TextColor(colors.text_color),
                        ));
                    });
            });
            toprow.with_children(|builder| {
                builder.spawn(Node {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder
                    .spawn(Node { ..default() })
                    .with_children(|innerbuilder| {
                        innerbuilder
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(30.0),
                                    height: Val::Px(30.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    align_content: AlignContent::Center,
                                    justify_items: JustifyItems::Center,
                                    margin: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                BackgroundColor(colors.red_color.into()),
                                BorderRadius::all(Val::Px(3.0)),
                                UiInteractionBtn,
                                GoToBackBtn,
                            ))
                            .with_children(|ccbuilder| {
                                ccbuilder.spawn((
                                    Text::new("X"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 30.0,
                                        font_smoothing: FontSmoothing::AntiAliased,
                                    },
                                    TextColor(colors.text_color),
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
                Node {
                    width: Val::Percent(100.0),
                    //height: Val::Percent(23.0),
                    display: Display::Grid,
                    justify_items: JustifyItems::Center,
                    border: UiRect::all(Val::Px(5.0)),
                    margin: UiRect::top(Val::Px(4.0)),
                    ..default()
                },
                BorderColor(colors.node_color_lighter),
                BorderRadius::all(Val::Px(4.0)),
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
        .spawn((
            Node {
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
                ..default()
            },
            BorderRadius::all(Val::Px(3.0)),
            BackgroundColor(node_color),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
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
                        ..default()
                    },
                    BackgroundColor(button_color.into()),
                    BorderRadius::all(Val::Px(8.0)),
                    UiInteractionBtn,
                    GoToTextBoxButton,
                    EditabledTextBox,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        Text::new(button_text.clone()),
                        TextFont {
                            font: font.clone(),
                            font_size,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(DEFAULT_NO_PICK_COLOR.into()),
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
        .spawn(Node {
            display: Display::Flex,
            justify_items: JustifyItems::Start,
            align_items: AlignItems::Center,
            align_content: AlignContent::Start,
            margin: UiRect::vertical(Val::Px(16.0)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        min_width: Val::Px(80.0),
                        min_height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(colors.button_color.into()),
                    BorderColor(colors.text_color),
                    BorderRadius::all(Val::Px(8.0)),
                    UiInteractionBtn,
                    GoToGoBtn,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        Text::new(button_text),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.0,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(colors.text_color),
                    ));
                });
        });
}
