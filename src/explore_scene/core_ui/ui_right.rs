use bevy::{color::palettes::css::WHITE, prelude::*, text::FontSmoothing};

use crate::{
    componenty::{
        DrawBtn, DrawBtnImage, GoToBtn, HideBuilding, HideText, MagnifyToggleBlockTime,
        MagnifyToggleChild, MagnifyToggleDifficulty, MagnifyToggleExcessWork, MagnifyToggleFee,
        MagnifyToggleLeadingZeros, MagnifyToggleParentBtn, MagnifyToggleSizeBytes,
        MagnifyToggleSizeWeight, MagnifyToggleTxCount, MagnifyToggleVersion, ShowColors,
        ShowValues, Toggle1Btn, Toggle1BtnText, Toggle2Btn, Toggle2BtnText, Toggle3Btn,
        Toggle3BtnText, Toggle4Btn, Toggle4BtnText, ToggleGameButton, ToggleParent,
        UiInteractionBtn, UiOverlayingExplorerButton, UiSideNode,
    },
    consty::{UI_ICON_SIZE, UI_SMALL_TEXT_SIZE},
    eventy::{ToggleBuildings, ToggleColors, ToggleText},
    resourcey::{BlockExplorerCount, ColorMapToggle, ColorPalette, MapTileMode, ToggleMap},
    structy::TileTextType,
};

use super::{
    components::ExplorerUiNodeRight,
    event::{HideGameToggleChildren, HideMagnifyToggleChildren},
};

pub fn right_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeRight>>,
    block_explorer_count: Res<BlockExplorerCount>,
) {
    for ent in placement_query.iter() {
        let mut side_parent = commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderRadius::all(Val::Px(4.0)),
            UiSideNode,
        ));

        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        // draw button
        side_parent.with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
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
                    ImageNode::new(asset_server.load("ui/palette_120x120.png")),
                    Visibility::Visible,
                    UiInteractionBtn,
                    DrawBtn,
                    UiOverlayingExplorerButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            height: Val::Px(UI_ICON_SIZE),
                            width: Val::Px(UI_ICON_SIZE),
                            ..default()
                        },
                        ImageNode::new(asset_server.load("ui/blank_120x120.png")),
                        DrawBtnImage,
                    ));
                });
        });
        // goto button
        side_parent.with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
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
                    Visibility::Visible,
                    BackgroundColor(WHITE.into()),
                    BorderColor(Color::BLACK),
                    BorderRadius::all(Val::Px(16.0)),
                    UiInteractionBtn,
                    GoToBtn,
                    UiOverlayingExplorerButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            height: Val::Px(UI_ICON_SIZE),
                            width: Val::Px(UI_ICON_SIZE),
                            ..default()
                        },
                        ImageNode::new(asset_server.load("ui/goto2_120x120.png")),
                    ));
                });
        });
        //toggle buttons
        side_parent.with_children(|parent| {
            parent.spawn((
                Button,
                Node {
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
                ImageNode::new(asset_server.load("ui/toggle_120x120.png")),
                Visibility::Visible,
                BorderRadius::all(Val::Px(16.0)),
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
                "Show Land",
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

        if block_explorer_count.0 > 0 {
            // bitcoin filter toggle

            side_parent
                .with_children(|parent| {
                    parent.spawn((
                        Button,
                        Node {
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
                        Visibility::Visible,
                        ImageNode::new(asset_server.load("ui/bitcoinmagnify_120x120.png")),
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
            Button,
            Node {
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
            BackgroundColor(colors.button_color.into()),
            BorderColor(colors.node_color),
            BorderRadius::all(Val::Px(8.0)),
            UiInteractionBtn,
            toggle_btn_type,
            ToggleGameButton,
            toggle_btn_position,
            UiOverlayingExplorerButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(btn_text),
                TextFont {
                    font,
                    font_size: UI_SMALL_TEXT_SIZE,
                    font_smoothing: FontSmoothing::AntiAliased,
                },
                TextColor(colors.text_color),
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
            Button,
            Node {
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
            BackgroundColor(colors.button_color.into()),
            BorderColor(colors.node_color),
            BorderRadius::all(Val::Px(8.0)),
            UiInteractionBtn,
            toggle_type,
            MagnifyToggleChild(btn_text.to_string()),
            UiOverlayingExplorerButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(btn_text),
                TextFont {
                    font,
                    font_size: UI_SMALL_TEXT_SIZE,
                    font_smoothing: FontSmoothing::AntiAliased,
                },
                TextColor(colors.text_color),
            ));
        });
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_button_system(
    // mut mouse: ResMut<ButtonInput<MouseButton>>,
    // mut touches: ResMut<Touches>,
    mut interaction_query: Query<
        (&Interaction, &mut ImageNode),
        (
            Changed<Interaction>,
            (With<ToggleParent>, Without<ToggleGameButton>),
        ),
    >,
    mut child_btns: Query<&mut Node, With<ToggleGameButton>>,
    //mut toggle_visible: ResMut<ToggleVisible>,
    colors: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    mut hide_blockchain_togggle: EventWriter<HideMagnifyToggleChildren>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let default_bg_color = color;
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                for mut style in child_btns.iter_mut() {
                    if style.display != Display::None {
                        style.display = Display::None;
                    } else {
                        style.display = Display::Flex;
                    }
                }

                *color = ImageNode::new(asset_server.load("ui/toggle_120x120.png"));
                hide_blockchain_togggle.send(HideMagnifyToggleChildren);
            }
            Interaction::Hovered => {
                *color = ImageNode::new(asset_server.load("ui/toggle_120x120.png"))
                    .with_color(colors.accent_color)
                //colors.accent_color.into();
            }
            Interaction::None => {
                *color = ImageNode::new(asset_server.load("ui/toggle_120x120.png"))
                    .with_color(colors.light_color);
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
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

                match text.as_str() {
                    "Show Buildings" => {
                        **text = "Hide Buildings".to_string();
                        *toggle_map.0.get_mut("showbuildings").unwrap() = false;
                        toggle.send(ToggleBuildings);
                    }
                    "Hide Buildings" => {
                        **text = "Show Buildings".to_string();
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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
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
    mut map_mode: ResMut<MapTileMode>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
                let mut text = text_query.get_single_mut().unwrap();

                match text.as_str() {
                    "Show Colors" => {
                        **text = "Show Land".to_string();
                        *toggle_map.0.get_mut("showcolors").unwrap() = false;
                        // let tiles = tile_map.to_tiledata_vec();
                        // update_tile_event.send(UpdateTileTextureEvent(tiles));
                        toggle.send(ToggleColors);
                        map_mode.0 = ColorMapToggle::GameColor;
                    }
                    "Show Land" => {
                        **text = "Show Colors".to_string();
                        *toggle_map.0.get_mut("showcolors").unwrap() = true;
                        toggle.send(ToggleColors);
                        // let tiles = tile_map.to_tiledata_vec();
                        // update_tile_event.send(UpdateTileTextureEvent(tiles));
                        map_mode.0 = ColorMapToggle::LandTile;
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

                match text3.as_str() {
                    "Show Values" => {
                        **text3 = "Show Heights".to_string();
                        *toggle_map.0.get_mut("showheights").unwrap() = true;
                        *toggle_map.0.get_mut("showvalues").unwrap() = false;
                        tile_text_type.send(ToggleText(TileTextType::Value));
                    }
                    "Show Heights" => {
                        **text3 = "Show Values".to_string();
                        *toggle_map.0.get_mut("showheights").unwrap() = false;
                        *toggle_map.0.get_mut("showvalues").unwrap() = true;
                        tile_text_type.send(ToggleText(TileTextType::Height));
                    }
                    _ => {
                        info!("wut bccc3");
                    }
                };
                **text4 = "Hide Text".to_string();
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

                match text.as_str() {
                    "Hide Text" => {
                        **text = "Show Text".to_string();
                        *toggle_map.0.get_mut("showtext").unwrap() = true;
                        tile_text_type.send(ToggleText(TileTextType::Blank));
                    }
                    "Show Text" => {
                        **text = "Hide Text".to_string();
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
        (&Interaction, &mut ImageNode),
        (
            Changed<Interaction>,
            (With<MagnifyToggleParentBtn>, Without<MagnifyToggleChild>),
        ),
    >,
    mut child_btns: Query<&mut Node, With<MagnifyToggleChild>>,
    colors: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
    mut hide_game_togggle: EventWriter<HideGameToggleChildren>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let default_bg_color = color;
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = ImageNode::new(asset_server.load("ui/bitcoinmagnify_120x120.png"));
                for mut style in child_btns.iter_mut() {
                    if style.display == Display::None {
                        style.display = Display::Flex;
                        hide_game_togggle.send(HideGameToggleChildren);
                    } else {
                        style.display = Display::None;
                    }
                }
            }
            Interaction::Hovered => {
                *color = ImageNode::new(asset_server.load("ui/bitcoinmagnify_120x120.png"))
                    .with_color(colors.accent_color)
            }
            Interaction::None => {
                *color = ImageNode::new(asset_server.load("ui/bitcoinmagnify_120x120.png"))
                    .with_color(colors.light_color);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn magnify_child_btn(
    mut interaction_query: Query<
        (&Interaction, &MagnifyToggleChild, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut toggle: EventWriter<ToggleColors>,
    colors: Res<ColorPalette>,
    mut map_mode: ResMut<MapTileMode>,
) {
    for (interaction, magnify_type, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();

                match magnify_type.0.as_str() {
                    "Block Fees" => map_mode.0 = ColorMapToggle::Fee,
                    "Block Time" => map_mode.0 = ColorMapToggle::BlockTime,
                    "Tx Count" => map_mode.0 = ColorMapToggle::TxCount,
                    "Size Bytes" => map_mode.0 = ColorMapToggle::Byte,
                    "Size Weight" => map_mode.0 = ColorMapToggle::Weight,
                    "Target Difficulty" => map_mode.0 = ColorMapToggle::TargetDifficulty,
                    "Leading Zeros" => map_mode.0 = ColorMapToggle::LeadingZeros,
                    "Excess Work" => map_mode.0 = ColorMapToggle::ExcessWork,
                    "Version" => map_mode.0 = ColorMapToggle::Version,
                    _ => map_mode.0 = ColorMapToggle::GameColor,
                };

                toggle.send(ToggleColors);
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_game_children(
    mut event: EventReader<HideGameToggleChildren>,
    mut child_btns: Query<&mut Node, With<ToggleGameButton>>,
) {
    for _e in event.read() {
        for mut style in child_btns.iter_mut() {
            if style.display != Display::None {
                style.display = Display::None;
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn toggle_magnify_children(
    mut event: EventReader<HideMagnifyToggleChildren>,
    mut child_btns: Query<&mut Node, With<MagnifyToggleChild>>,
) {
    for _e in event.read() {
        for mut style in child_btns.iter_mut() {
            if style.display != Display::None {
                style.display = Display::None;
            }
        }
    }
}
