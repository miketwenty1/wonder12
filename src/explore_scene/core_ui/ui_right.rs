use bevy::{
    color::palettes::css::WHITE,
    ecs::system::{EntityCommand, EntityCommands},
    prelude::*,
};

use crate::{
    componenty::{
        DrawBtn, DrawBtnImage, GoToBtn, HideBuilding, HideText, MagnifyToggleBlockTime,
        MagnifyToggleChild, MagnifyToggleDifficulty, MagnifyToggleExcessWork, MagnifyToggleFee,
        MagnifyToggleLeadingZeros, MagnifyToggleParentBtn, MagnifyToggleSizeBytes,
        MagnifyToggleSizeWeight, MagnifyToggleTxCount, MagnifyToggleVersion, ShowColors,
        ShowValues, Toggle1Btn, Toggle1BtnText, Toggle2Btn, Toggle2BtnText, Toggle3Btn,
        Toggle3BtnText, Toggle4Btn, Toggle4BtnText, ToggleButton, ToggleParent, UiInteractionBtn,
        UiOverlayingExplorerButton, UiSideNode,
    },
    consty::{UI_ICON_SIZE, UI_SMALL_TEXT_SIZE},
    eventy::{ToggleBuildings, ToggleColors, ToggleText},
    resourcey::{BlockExplorer, ColorPalette, ToggleMap, ToggleVisible},
    structy::TileTextType,
};

use super::components::ExplorerUiNodeRight;

pub fn right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeRight>>,
    blockexplorer_bool: Res<BlockExplorer>,
) {
    for ent in placement_query.iter() {
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
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            UiSideNode,
        ));

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        // draw button
        side_parent.with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(UI_ICON_SIZE), // to make it a square.
                            height: Val::Px(UI_ICON_SIZE),
                            //border: UiRect::all(Val::Px(2.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(3.0)),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("ui/palette_120x120.png")),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    UiInteractionBtn,
                    DrawBtn,
                    UiOverlayingExplorerButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageBundle {
                            style: Style {
                                height: Val::Px(UI_ICON_SIZE),
                                width: Val::Px(UI_ICON_SIZE),
                                ..default()
                            },
                            image: UiImage::new(asset_server.load("ui/blank_120x120.png")),
                            ..default()
                        },
                        DrawBtnImage,
                    ));
                });
        });
        // goto button
        side_parent.with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(UI_ICON_SIZE),
                            height: Val::Px(UI_ICON_SIZE),
                            border: UiRect::all(Val::Px(2.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(3.0)),
                            ..default()
                        },
                        border_radius: BorderRadius::all(Val::Px(16.0)),
                        //image: UiImage::new(asset_server.load("ui/goto2_120x120.png")),
                        border_color: BorderColor(Color::BLACK),
                        background_color: BackgroundColor(WHITE.into()),
                        visibility: Visibility::Visible,
                        ..default()
                    },
                    UiInteractionBtn,
                    GoToBtn,
                    UiOverlayingExplorerButton,
                ))
                .with_children(|parent| {
                    parent.spawn((ImageBundle {
                        style: Style {
                            height: Val::Px(UI_ICON_SIZE),
                            width: Val::Px(UI_ICON_SIZE),
                            ..default()
                        },
                        image: UiImage::new(asset_server.load("ui/goto2_120x120.png")),
                        ..default()
                    },));
                });
        });
        //toggle buttons
        side_parent.with_children(|parent| {
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(UI_ICON_SIZE),
                        height: Val::Px(UI_ICON_SIZE),
                        //border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(3.0)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(16.0)),
                    image: UiImage::new(asset_server.load("ui/toggle_120x120.png")),
                    visibility: Visibility::Visible,
                    ..default()
                },
                UiInteractionBtn,
                ToggleParent,
                UiOverlayingExplorerButton,
            ));

            spawn_game_toggle_button(
                parent,
                Toggle1Btn,
                HideBuilding,
                Toggle1BtnText,
                "Hide Buildings",
                colors.clone(),
                font.clone(),
            );
            spawn_game_toggle_button(
                parent,
                Toggle2Btn,
                ShowColors,
                Toggle2BtnText,
                "Hide Colors",
                colors.clone(),
                font.clone(),
            );
            spawn_game_toggle_button(
                parent,
                Toggle3Btn,
                ShowValues,
                Toggle3BtnText,
                "Show Values",
                colors.clone(),
                font.clone(),
            );
            spawn_game_toggle_button(
                parent,
                Toggle4Btn,
                HideText,
                Toggle4BtnText,
                "Hide Text",
                colors.clone(),
                font.clone(),
            );
        });

        if blockexplorer_bool.0 {
            // bitcoin filter toggle

            side_parent
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(UI_ICON_SIZE),
                                height: Val::Px(UI_ICON_SIZE),
                                //border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                margin: UiRect::top(Val::Px(3.0)),
                                ..default()
                            },
                            //border_radius: BorderRadius::all(Val::Px(16.0)),
                            image: UiImage::new(asset_server.load("ui/bitcoinmagnify_120x120.png")),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        UiInteractionBtn,
                        MagnifyToggleParentBtn,
                        UiOverlayingExplorerButton,
                    ));
                })
                .with_children(|magnify_parent| {
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleFee,
                        "Block Fees",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleBlockTime,
                        "Block Time",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleTxCount,
                        "Tx Count",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleSizeBytes,
                        "Size Bytes",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleSizeWeight,
                        "Size Weight",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleDifficulty,
                        "Target Difficulty",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleLeadingZeros,
                        "Leading Zeros",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleExcessWork,
                        "Excess Work",
                        colors.clone(),
                        font.clone(),
                    );
                    spawn_magnify_toggle_button(
                        magnify_parent,
                        MagnifyToggleVersion,
                        "Version",
                        colors.clone(),
                        font.clone(),
                    );
                });
        }

        side_parent.set_parent(ent);
    }
}

#[warn(clippy::too_many_arguments)]
fn spawn_game_toggle_button<T: Component, U: Component, V: Component>(
    parent: &mut ChildBuilder,
    toggle_btn_position: T,
    toggle_btn_type: U,
    toggle_btn_text_position: V,
    btn_text: &str,
    colors: ColorPalette,
    font: Handle<Font>,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(UI_ICON_SIZE),
                    height: Val::Px(UI_ICON_SIZE),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    align_content: AlignContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(8.0)),
                border_color: BorderColor(colors.node_color),
                background_color: colors.button_color.into(),
                ..default()
            },
            UiInteractionBtn,
            toggle_btn_type,
            ToggleButton,
            toggle_btn_position,
            UiOverlayingExplorerButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    btn_text,
                    TextStyle {
                        font,
                        font_size: UI_SMALL_TEXT_SIZE,
                        color: colors.text_color,
                    },
                ),
                toggle_btn_text_position,
            ));
        });
}

fn spawn_magnify_toggle_button<T: Component>(
    parent: &mut ChildBuilder,
    toggle_type: T,
    btn_text: &str,
    colors: ColorPalette,
    font: Handle<Font>,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(UI_ICON_SIZE),
                    height: Val::Px(UI_ICON_SIZE),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    align_content: AlignContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    display: Display::None,
                    ..default()
                },
                border_radius: BorderRadius::all(Val::Px(8.0)),
                border_color: BorderColor(colors.node_color),
                background_color: colors.button_color.into(),
                ..default()
            },
            UiInteractionBtn,
            toggle_type,
            MagnifyToggleChild,
            UiOverlayingExplorerButton,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                btn_text,
                TextStyle {
                    font,
                    font_size: UI_SMALL_TEXT_SIZE,
                    color: colors.text_color,
                },
            ),));
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_button_system(
    // mut mouse: ResMut<ButtonInput<MouseButton>>,
    // mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
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
        Query<&mut Style, With<Toggle1Btn>>,
        Query<&mut Style, With<Toggle2Btn>>,
        Query<&mut Style, With<Toggle3Btn>>,
        Query<&mut Style, With<Toggle4Btn>>,
    )>,
    mut toggle_visible: ResMut<ToggleVisible>,
    colors: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let default_bg_color = color;
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = UiImage::new(asset_server.load("ui/toggle_120x120.png"));
                //game_state.set(DisplayBuyUiState::On);
                if toggle_visible.0 {
                    for mut style in param_set.p0().iter_mut() {
                        style.display = Display::None;
                    }
                    for mut style in param_set.p1().iter_mut() {
                        style.display = Display::None;
                    }
                    for mut style in param_set.p2().iter_mut() {
                        style.display = Display::None;
                    }
                    for mut style in param_set.p3().iter_mut() {
                        style.display = Display::None;
                    }
                    toggle_visible.0 = false;
                } else {
                    for mut style in param_set.p0().iter_mut() {
                        style.display = Display::Flex;
                    }

                    for mut style in param_set.p1().iter_mut() {
                        style.display = Display::Flex;
                    }

                    for mut style in param_set.p2().iter_mut() {
                        style.display = Display::Flex;
                    }

                    for mut style in param_set.p3().iter_mut() {
                        style.display = Display::Flex;
                    }

                    toggle_visible.0 = true;
                }
            }
            Interaction::Hovered => {
                *color = UiImage::new(asset_server.load("ui/toggle_120x120.png"))
                    .with_color(colors.accent_color)
                //colors.accent_color.into();
            }
            Interaction::None => {
                *color = UiImage::new(asset_server.load("ui/toggle_120x120.png"))
                    .with_color(colors.light_color);
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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_magnify_button_system(
    // mut mouse: ResMut<ButtonInput<MouseButton>>,
    // mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (
            Changed<Interaction>,
            (With<MagnifyToggleParentBtn>, Without<MagnifyToggleChild>),
        ),
    >,
    mut child_btns: Query<&mut Style, With<MagnifyToggleChild>>,
    colors: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let default_bg_color = color;
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = UiImage::new(asset_server.load("ui/bitcoinmagnify_120x120.png"));
                for mut style in child_btns.iter_mut() {
                    if style.display == Display::None {
                        style.display = Display::Flex;
                    } else {
                        style.display = Display::None;
                    }
                }
            }
            Interaction::Hovered => {
                *color = UiImage::new(asset_server.load("ui/bitcoinmagnify_120x120.png"))
                    .with_color(colors.accent_color)
            }
            Interaction::None => {
                *color = UiImage::new(asset_server.load("ui/bitcoinmagnify_120x120.png"))
                    .with_color(colors.light_color);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn fees_map_btn(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MagnifyToggleFee>)>,
    mut tile_text_type: EventWriter<ToggleText>,
    colors: Res<ColorPalette>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
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
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
