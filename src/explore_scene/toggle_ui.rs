use bevy::prelude::*;

const TOGGLE_FONT_PARENT_SIZE: f32 = 20.0;
const TOGGLE_PARENT_BTN_WIDTH: f32 = 90.0;
const TOGGLE_PARENT_BTN_HEIGHT: f32 = 55.0;

const TOGGLE_FONT_CHILD_SIZE: f32 = 20.0;
const TOGGLE_CHILD_BTN_WIDTH: f32 = 85.0;
const TOGGLE_CHILD_BTN_HEIGHT: f32 = 55.0;

use crate::{
    componenty::{
        HideBuilding, HideBuildingText, HideColors, HideText, HideTextText, ShowBuilding,
        ShowColors, ShowColorsText, ShowHeights, ShowText, ShowValues, ShowValuesText, Toggle1Btn,
        Toggle1BtnText, Toggle2Btn, Toggle2BtnText, Toggle3Btn, Toggle3BtnText, Toggle4Btn,
        Toggle4BtnText, ToggleButton, ToggleParent, UiToggle,
    },
    consty::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    resourcey::ToggleMap,
};

pub fn setup_toggle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    toggle_map: Res<ToggleMap>,
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
            // parent
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
            //                 height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
            //                 border: UiRect::all(Val::Px(5.0)),
            //                 // horizontally center child text
            //                 justify_content: JustifyContent::Center,
            //                 // vertically center child text
            //                 align_items: AlignItems::Center,
            //                 ..default()
            //             },
            //             border_color: BorderColor(Color::BLACK),
            //             background_color: NORMAL_BUTTON.into(),
            //             visibility: Visibility::Hidden,
            //             ..default()
            //         },
            //         ShowBuilding,
            //         ToggleButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle::from_section(
            //             "Show Buildings",
            //             TextStyle {
            //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                 font_size: TOGGLE_FONT_CHILD_SIZE,
            //                 color: Color::rgb(0.9, 0.9, 0.9),
            //             },
            //         ));
            //     });
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
            // parent
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
            //                 height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
            //                 border: UiRect::all(Val::Px(5.0)),
            //                 // horizontally center child text
            //                 justify_content: JustifyContent::Center,
            //                 // vertically center child text
            //                 align_items: AlignItems::Center,
            //                 ..default()
            //             },
            //             border_color: BorderColor(Color::BLACK),
            //             background_color: NORMAL_BUTTON.into(),
            //             visibility: Visibility::Hidden,
            //             ..default()
            //         },
            //         HideColors,
            //         ToggleButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle::from_section(
            //             "Hide Colors",
            //             TextStyle {
            //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                 font_size: TOGGLE_FONT_CHILD_SIZE,
            //                 color: Color::rgb(0.9, 0.9, 0.9),
            //             },
            //         ));
            //     });
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
            // parent
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
            //                 height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
            //                 border: UiRect::all(Val::Px(5.0)),
            //                 // horizontally center child text
            //                 justify_content: JustifyContent::Center,
            //                 // vertically center child text
            //                 align_items: AlignItems::Center,
            //                 ..default()
            //             },
            //             border_color: BorderColor(Color::BLACK),
            //             background_color: NORMAL_BUTTON.into(),
            //             visibility: Visibility::Hidden,
            //             ..default()
            //         },
            //         ShowHeights,
            //         ToggleButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle::from_section(
            //             "Show Heights",
            //             TextStyle {
            //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                 font_size: TOGGLE_FONT_CHILD_SIZE,
            //                 color: Color::rgb(0.9, 0.9, 0.9),
            //             },
            //         ));
            //     });
            // parent
            //     .spawn((
            //         ButtonBundle {
            //             style: Style {
            //                 width: Val::Px(TOGGLE_CHILD_BTN_WIDTH),
            //                 height: Val::Px(TOGGLE_CHILD_BTN_HEIGHT),
            //                 border: UiRect::all(Val::Px(5.0)),
            //                 // horizontally center child text
            //                 justify_content: JustifyContent::Center,
            //                 // vertically center child text
            //                 align_items: AlignItems::Center,
            //                 ..default()
            //             },
            //             border_color: BorderColor(Color::BLACK),
            //             background_color: NORMAL_BUTTON.into(),
            //             visibility: Visibility::Hidden,
            //             ..default()
            //         },
            //         ShowText,
            //         ToggleButton,
            //     ))
            //     .with_children(|parent| {
            //         parent.spawn(TextBundle::from_section(
            //             "Show Text",
            //             TextStyle {
            //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            //                 font_size: TOGGLE_FONT_CHILD_SIZE,
            //                 color: Color::rgb(0.9, 0.9, 0.9),
            //             },
            //         ));
            //     });
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
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (
                With<ToggleParent>,
                Without<HideBuilding>,
                Without<ShowBuilding>,
                Without<ShowColors>,
                Without<HideColors>,
                Without<ShowValues>,
                Without<ShowHeights>,
                Without<ShowText>,
                Without<HideText>,
            ),
        ),
    >,
    mut param_set: ParamSet<(
        Query<&mut Visibility, With<HideBuilding>>,
        Query<&mut Visibility, With<ShowBuilding>>,
        Query<&mut Visibility, With<ShowColors>>,
        Query<&mut Visibility, With<HideColors>>,
        Query<&mut Visibility, With<ShowValues>>,
        Query<&mut Visibility, With<ShowHeights>>,
        Query<&mut Visibility, With<ShowText>>,
        Query<&mut Visibility, With<HideText>>,
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
    toggle_map: Res<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby");
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
                    for mut btn_vis in param_set.p4().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    for mut btn_vis in param_set.p5().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    for mut btn_vis in param_set.p6().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    for mut btn_vis in param_set.p7().iter_mut() {
                        *btn_vis = Visibility::Hidden;
                    }
                    *toggle_visible = false;
                } else {
                    if *toggle_map.0.get("hidebuildings").unwrap() {
                        for mut btn_vis in param_set.p0().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("showbuildings").unwrap() {
                        for mut btn_vis in param_set.p1().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("showcolors").unwrap() {
                        for mut btn_vis in param_set.p2().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("hidecolors").unwrap() {
                        for mut btn_vis in param_set.p3().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("showvalues").unwrap() {
                        for mut btn_vis in param_set.p4().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("showheights").unwrap() {
                        for mut btn_vis in param_set.p5().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("showtext").unwrap() {
                        for mut btn_vis in param_set.p6().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
                    }
                    if *toggle_map.0.get("hidetext").unwrap() {
                        for mut btn_vis in param_set.p7().iter_mut() {
                            *btn_vis = Visibility::Visible;
                        }
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
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle1Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle1BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Show Buildings" => {
                        text.sections[0].value = "Hide Buildings".to_string();
                    }
                    "Hide Buildings" => {
                        text.sections[0].value = "Show Buildings".to_string();
                    }
                    _ => {
                        info!("wut b2323");
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
pub fn toggle_button_sub_system_show_buildings(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
pub fn toggle_button_sub_system_show_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
pub fn toggle_button_sub_system_hide_colors(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
pub fn toggle_button_sub_system_show_values(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
pub fn toggle_button_sub_system_show_heights(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
pub fn toggle_button_sub_system_show_text(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
pub fn toggle_button_sub_system_hide_text(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<ToggleButton>,
            Without<ToggleParent>,
        ),
    >,
    mut toggle_map: ResMut<ToggleMap>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayUiState::On);
                info!("go baby btn");
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
