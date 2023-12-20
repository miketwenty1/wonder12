use bevy::prelude::*;

const TOGGLE_FONT_PARENT_SIZE: f32 = 22.0;
const TOGGLE_PARENT_BTN_WIDTH: f32 = 95.0;
const TOGGLE_PARENT_BTN_HEIGHT: f32 = 60.0;

const TOGGLE_FONT_CHILD_SIZE: f32 = 20.0;
const TOGGLE_CHILD_BTN_WIDTH: f32 = 90.0;
const TOGGLE_CHILD_BTN_HEIGHT: f32 = 60.0;

use crate::{
    componenty::{
        HideBuilding, HideText, HideTextText, ShowColors, ShowValues, Toggle1Btn, Toggle1BtnText,
        Toggle2Btn, Toggle2BtnText, Toggle3Btn, Toggle3BtnText, Toggle4Btn, Toggle4BtnText,
        ToggleButton, ToggleParent, UiToggle,
    },
    consty::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    eventy::{ToggleBuildings, ToggleColors, ToggleText},
    resourcey::ToggleMap,
    structy::TileTextType,
};

pub fn setup_toggle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    //toggle_map: Res<ToggleMap>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            UiToggle,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(TOGGLE_PARENT_BTN_WIDTH),
                            height: Val::Px(TOGGLE_PARENT_BTN_HEIGHT),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    ToggleParent,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Toggle",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: TOGGLE_FONT_PARENT_SIZE,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
                            height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    HideBuilding,
                    Toggle1Btn,
                    ToggleButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Hide Buildings",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: TOGGLE_FONT_CHILD_SIZE,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        Toggle1BtnText,
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
                            height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ShowColors,
                    ToggleButton,
                    Toggle2Btn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Show Colors",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: TOGGLE_FONT_CHILD_SIZE,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        Toggle2BtnText,
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
                            height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ShowValues,
                    ToggleButton,
                    Toggle3Btn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Show Values",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: TOGGLE_FONT_CHILD_SIZE,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        Toggle3BtnText,
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
                            height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    HideText,
                    ToggleButton,
                    Toggle4Btn,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Hide Text",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: TOGGLE_FONT_CHILD_SIZE,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        HideTextText,
                        Toggle4BtnText,
                    ));
                });
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_button_system(
    mut mouse: ResMut<Input<MouseButton>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (
                With<ToggleParent>,
                Without<Toggle1Btn>,
                Without<Toggle2Btn>,
                Without<Toggle3Btn>,
                Without<Toggle4Btn>,
            ),
        ),
    >,
    mut param_set: ParamSet<(
        Query<&mut Visibility, With<Toggle1Btn>>,
        Query<&mut Visibility, With<Toggle2Btn>>,
        Query<&mut Visibility, With<Toggle3Btn>>,
        Query<&mut Visibility, With<Toggle4Btn>>,
        // Query<&mut Visibility, With<ShowValues>>,
        // Query<&mut Visibility, With<ShowHeights>>,
        // Query<&mut Visibility, With<ShowText>>,
        // Query<&mut Visibility, With<HideText>>,
    )>,
    //mut toggle_subbtn_query: Query<&mut Visibility, (With<ToggleParent>, Without<ToggleParent>)>,
    // mut toggle_subbtn_query_hide_buildings: Query<&mut Visibility, With<HideBuilding>>,
    // mut toggle_subbtn_query_show_buildings: Query<&mut Visibility, With<ShowBuilding>>,
    // mut toggle_subbtn_query_show_colors: Query<&mut Visibility, With<ShowColors>>,
    // mut toggle_subbtn_query_hide_colors: Query<&mut Visibility, With<HideColors>>,
    // mut toggle_subbtn_query_show_values: Query<&mut Visibility, With<ShowValues>>,
    // mut toggle_subbtn_query_show_heights: Query<&mut Visibility, With<ShowHeights>>,
    // mut toggle_subbtn_query_show_text: Query<&mut Visibility, With<ShowText>>,
    // mut toggle_subbtn_query_hide_text: Query<&mut Visibility, With<HideText>>,
    mut toggle_visible: Local<bool>,
    //toggle_map: Res<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayBuyUiState::On);
                if *toggle_visible {
                    for mut btn_vis in param_set.p0().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    for mut btn_vis in param_set.p1().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    for mut btn_vis in param_set.p2().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    for mut btn_vis in param_set.p3().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    *toggle_visible = false;
                } else {
                    for mut btn_vis in param_set.p0().iter_mut() {
                        *btn_vis = Visibility::Visible;
                    }

                    for mut btn_vis in param_set.p1().iter_mut() {
                        *btn_vis = Visibility::Visible;
                    }

                    for mut btn_vis in param_set.p2().iter_mut() {
                        *btn_vis = Visibility::Visible;
                    }

                    for mut btn_vis in param_set.p3().iter_mut() {
                        *btn_vis = Visibility::Visible;
                    }

                    *toggle_visible = true;
                }
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
pub fn toggle_button_sub_system_toggle1(
    mut mouse: ResMut<Input<MouseButton>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle1Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle1BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut toggle: EventWriter<ToggleBuildings>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Show Buildings" => {
                        text.sections[0].value = "Hide Buildings".to_string();
                        *toggle_map.0.get_mut("showbuildings").unwrap() = false;
                        *toggle_map.0.get_mut("hidebuildings").unwrap() = true;
                        toggle.send(ToggleBuildings);
                    }
                    "Hide Buildings" => {
                        text.sections[0].value = "Show Buildings".to_string();
                        *toggle_map.0.get_mut("showbuildings").unwrap() = true;
                        *toggle_map.0.get_mut("hidebuildings").unwrap() = false;
                        toggle.send(ToggleBuildings);
                    }
                    _ => {
                        info!("wut bccc1");
                    }
                };
                *color = PRESSED_BUTTON.into();
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
pub fn toggle_button_sub_system_toggle2(
    mut mouse: ResMut<Input<MouseButton>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle2Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle2BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut toggle: EventWriter<ToggleColors>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Show Colors" => {
                        text.sections[0].value = "Hide Colors".to_string();
                        *toggle_map.0.get_mut("hidecolors").unwrap() = true;
                        *toggle_map.0.get_mut("showcolors").unwrap() = false;
                        toggle.send(ToggleColors);
                    }
                    "Hide Colors" => {
                        text.sections[0].value = "Show Colors".to_string();
                        *toggle_map.0.get_mut("hidecolors").unwrap() = false;
                        *toggle_map.0.get_mut("showcolors").unwrap() = true;
                        toggle.send(ToggleColors);
                    }
                    _ => {
                        info!("wut bccc2");
                    }
                };
                *color = PRESSED_BUTTON.into();
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
pub fn toggle_button_sub_system_toggle3(
    mut mouse: ResMut<Input<MouseButton>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle3Btn>),
    >,
    mut text_query3: Query<&mut Text, With<Toggle3BtnText>>,
    mut text_query4: Query<&mut Text, (With<Toggle4BtnText>, Without<Toggle3BtnText>)>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut tile_text_type: EventWriter<ToggleText>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                let mut text3 = text_query3.get_single_mut().unwrap();
                let mut text4 = text_query4.get_single_mut().unwrap();

                match text3.sections[0].value.as_str() {
                    "Show Values" => {
                        text3.sections[0].value = "Show Heights".to_string();
                        *toggle_map.0.get_mut("showheights").unwrap() = true;
                        *toggle_map.0.get_mut("showvalues").unwrap() = false;
                        tile_text_type.send(ToggleText(TileTextType::Value));
                    }
                    "Show Heights" => {
                        text3.sections[0].value = "Show Values".to_string();
                        *toggle_map.0.get_mut("showheights").unwrap() = false;
                        *toggle_map.0.get_mut("showvalues").unwrap() = true;
                        tile_text_type.send(ToggleText(TileTextType::Height));
                    }
                    _ => {
                        info!("wut bccc3");
                    }
                };
                text4.sections[0].value = "Hide Text".to_string();
                *toggle_map.0.get_mut("hidetext").unwrap() = true;
                *toggle_map.0.get_mut("showtext").unwrap() = false;
                *color = PRESSED_BUTTON.into();
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
pub fn toggle_button_sub_system_toggle4(
    mut mouse: ResMut<Input<MouseButton>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle4Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle4BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut tile_text_type: EventWriter<ToggleText>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Hide Text" => {
                        text.sections[0].value = "Show Text".to_string();
                        *toggle_map.0.get_mut("showtext").unwrap() = true;
                        *toggle_map.0.get_mut("hidetext").unwrap() = false;
                        tile_text_type.send(ToggleText(TileTextType::Blank));
                    }
                    "Show Text" => {
                        text.sections[0].value = "Hide Text".to_string();
                        *toggle_map.0.get_mut("showtext").unwrap() = false;
                        *toggle_map.0.get_mut("hidetext").unwrap() = true;
                        if *toggle_map.0.get("showvalues").unwrap() {
                            tile_text_type.send(ToggleText(TileTextType::Height));
                        } else {
                            tile_text_type.send(ToggleText(TileTextType::Value));
                        }
                    }
                    _ => {
                        info!("wut bccc4");
                    }
                };
                *color = PRESSED_BUTTON.into();
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
