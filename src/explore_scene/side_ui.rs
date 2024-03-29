use bevy::prelude::*;

const FONT_PARENT_SIZE: f32 = 22.0;
const PARENT_BTN_WIDTH: f32 = 95.0;
const PARENT_BTN_HEIGHT: f32 = 60.0;

const FONT_CHILD_SIZE: f32 = 20.0;
const CHILD_BTN_WIDTH: f32 = 90.0;
const CHILD_BTN_HEIGHT: f32 = 60.0;

use crate::{
    componenty::{
        GoToBtn, HideBuilding, HideText, HideTextText, ShowColors, ShowValues, Toggle1Btn,
        Toggle1BtnText, Toggle2Btn, Toggle2BtnText, Toggle3Btn, Toggle3BtnText, Toggle4Btn,
        Toggle4BtnText, ToggleButton, ToggleParent, UiOverlayingExplorerButton, UiSideNode,
    },
    eventy::{ToggleBuildings, ToggleColors, ToggleText},
    resourcey::{ColorPalette, ToggleMap, ToggleVisible},
    structy::TileTextType,
};

pub fn setup_side_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
) {
    let mut side_parent = commands.spawn((
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
        UiSideNode,
    ));

    // goto button
    side_parent.with_children(|parent| {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(PARENT_BTN_WIDTH),
                        height: Val::Px(PARENT_BTN_HEIGHT),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.text_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Visible,
                    ..default()
                },
                GoToBtn,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Go To",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_PARENT_SIZE,
                        color: colors.text_color,
                    },
                ));
            });
    });
    //toggle buttons
    side_parent.with_children(|parent| {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(PARENT_BTN_WIDTH),
                        height: Val::Px(PARENT_BTN_HEIGHT),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.text_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Visible,
                    ..default()
                },
                ToggleParent,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Toggle",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: FONT_PARENT_SIZE,
                        color: colors.text_color,
                    },
                ));
            });
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(CHILD_BTN_WIDTH),
                        height: Val::Px(CHILD_BTN_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.node_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HideBuilding,
                Toggle1Btn,
                ToggleButton,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Hide Buildings",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_CHILD_SIZE,
                            color: colors.text_color,
                        },
                    ),
                    Toggle1BtnText,
                ));
            });
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(CHILD_BTN_WIDTH),
                        height: Val::Px(CHILD_BTN_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.node_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ShowColors,
                ToggleButton,
                Toggle2Btn,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Hide Colors",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_CHILD_SIZE,
                            color: colors.text_color,
                        },
                    ),
                    Toggle2BtnText,
                ));
            });

        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(CHILD_BTN_WIDTH),
                        height: Val::Px(CHILD_BTN_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.node_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ShowValues,
                ToggleButton,
                Toggle3Btn,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Show Values",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_CHILD_SIZE,
                            color: colors.text_color,
                        },
                    ),
                    Toggle3BtnText,
                ));
            });
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(CHILD_BTN_WIDTH),
                        height: Val::Px(CHILD_BTN_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.node_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                HideText,
                ToggleButton,
                Toggle4Btn,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Hide Text",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: FONT_CHILD_SIZE,
                            color: colors.text_color,
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
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
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
    )>,

    //mut toggle_visible: Local<bool>,
    mut toggle_visible: ResMut<ToggleVisible>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
                //text.sections[0].value = button_text;
                *color = colors.light_color.into();
                //game_state.set(DisplayBuyUiState::On);
                if toggle_visible.0 {
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
                    toggle_visible.0 = false;
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

                    toggle_visible.0 = true;
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn toggle_button_sub_system_toggle1(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle1Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle1BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut toggle: EventWriter<ToggleBuildings>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Show Buildings" => {
                        text.sections[0].value = "Hide Buildings".to_string();
                        *toggle_map.0.get_mut("showbuildings").unwrap() = false;
                        toggle.send(ToggleBuildings);
                    }
                    "Hide Buildings" => {
                        text.sections[0].value = "Show Buildings".to_string();
                        *toggle_map.0.get_mut("showbuildings").unwrap() = true;
                        toggle.send(ToggleBuildings);
                    }
                    _ => {
                        info!("wut bccc1");
                    }
                };
                *color = colors.light_color.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn toggle_button_sub_system_toggle2(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle2Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle2BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut toggle: EventWriter<ToggleColors>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Show Colors" => {
                        text.sections[0].value = "Hide Colors".to_string();
                        *toggle_map.0.get_mut("showcolors").unwrap() = false;
                        toggle.send(ToggleColors);
                    }
                    "Hide Colors" => {
                        text.sections[0].value = "Show Colors".to_string();
                        *toggle_map.0.get_mut("showcolors").unwrap() = true;
                        toggle.send(ToggleColors);
                    }
                    _ => {
                        info!("wut bccc2");
                    }
                };
                *color = colors.light_color.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_button_sub_system_toggle3(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle3Btn>),
    >,
    mut text_query3: Query<&mut Text, With<Toggle3BtnText>>,
    mut text_query4: Query<&mut Text, (With<Toggle4BtnText>, Without<Toggle3BtnText>)>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut tile_text_type: EventWriter<ToggleText>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
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
                *toggle_map.0.get_mut("showtext").unwrap() = false;
                *color = colors.light_color.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}
#[allow(clippy::type_complexity)]
pub fn toggle_button_sub_system_toggle4(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Toggle4Btn>),
    >,
    mut text_query: Query<&mut Text, With<Toggle4BtnText>>,
    //mut toggle_subbtn_query: Query<&mut Visibility, With<Toggle1Btn>>,
    mut toggle_map: ResMut<ToggleMap>,
    mut tile_text_type: EventWriter<ToggleText>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
                let mut text = text_query.get_single_mut().unwrap();

                match text.sections[0].value.as_str() {
                    "Hide Text" => {
                        text.sections[0].value = "Show Text".to_string();
                        *toggle_map.0.get_mut("showtext").unwrap() = true;
                        tile_text_type.send(ToggleText(TileTextType::Blank));
                    }
                    "Show Text" => {
                        text.sections[0].value = "Hide Text".to_string();
                        *toggle_map.0.get_mut("showtext").unwrap() = false;
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
                *color = colors.light_color.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}
