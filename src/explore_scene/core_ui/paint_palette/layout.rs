use bevy::{color::palettes::css::DARK_GRAY, prelude::*, text::FontSmoothing};
use rand::seq::SliceRandom;

use crate::{
    componenty::UiInteractionBtn,
    consty::{UI_ICON_SIZE, UI_SMALL_TEXT_SIZE},
    explore_scene::core_ui::{
        components::ExplorerUiNodeLeft,
        inventory::state::InventoryUiState,
        paint_palette::component::{
            ColorPaletteViewText, ColorPaletteViewTextNode, PaletteBtn, PaletteEraserBtn,
            PaletteEyedropBtn, PaletteMoveBtn, PalettePencilBtn, PaletteTrashBtn,
        },
    },
    resourcey::ColorPalette,
};

use super::{
    component::{IndividualColorInPalette, PaintPaletteNode, PaletteViewHideBtn, ViewHideImg},
    resource::DefaultDrawColorPalette,
    state::ToolPaletteUiState,
};

pub fn spawn_layout(
    mut commands: Commands,
    // inventory_blocks: Res<UserInventoryBlocks>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeLeft>>,
    // mut inv_event: EventWriter<AddInventoryRow>,
    default_color_palette: Res<DefaultDrawColorPalette>,
) {
    for parent_node in placement_query.iter() {
        let mut overall_inventory_node = commands.spawn((
            Button,
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(4.0)),
                margin: UiRect::bottom(Val::Auto),
                //margin: UiRect::all(Val::Px(4.0)),
                row_gap: Val::Px(6.0),
                ..default()
            },
            BackgroundColor(colors.node_color_lighter),
            Visibility::Visible,
            BorderRadius::all(Val::Px(8.0)),
            PaintPaletteNode,
            UiInteractionBtn,
        ));

        overall_inventory_node.with_children(|builder| {
            ///////////
            // MOVE ROW
            ///////////
            builder
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(6.0),
                        column_gap: Val::Px(8.0),
                        padding: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(colors.node_color),
                    BorderRadius::all(Val::Px(8.0)),
                ))
                .with_children(|inner_builder| {
                    inner_builder
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 1.5), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 1.5),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Visibility::Visible,
                            BackgroundColor(colors.accent_color),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(8.0)),
                            UiInteractionBtn,
                            PalettePencilBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                ImageNode {
                                    image: asset_server.load("ui/pencil_120x120.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(UI_ICON_SIZE / 2.),
                                    width: Val::Px(UI_ICON_SIZE / 2.),
                                    ..Default::default()
                                },
                            ));
                        });

                    inner_builder
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 1.5), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 1.5),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Visibility::Visible,
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(40.0)),
                            UiInteractionBtn,
                            PaletteMoveBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                ImageNode {
                                    image: asset_server.load("ui/move_60x60.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(UI_ICON_SIZE / 2.),
                                    width: Val::Px(UI_ICON_SIZE / 2.),
                                    ..default()
                                },
                            ));
                        });
                });

            ///////////
            // TOOLS ROW
            ///////////
            builder
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        //row_gap: Val::Px(6.0),
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        flex_wrap: FlexWrap::Wrap,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(colors.node_color),
                ))
                .with_children(|inner_builder| {
                    inner_builder
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 2.), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 2.),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Visibility::Visible,
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(8.0)),
                            UiInteractionBtn,
                            PaletteEraserBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                ImageNode {
                                    image: asset_server.load("ui/eraser_60x60.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                            ));
                        });
                    inner_builder
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 2.), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 2.),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Visibility::Visible,
                            BackgroundColor(Color::WHITE),
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(8.0)),
                            UiInteractionBtn,
                            PaletteEyedropBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                ImageNode {
                                    image: asset_server.load("ui/eyedrop_60x60.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                            ));
                        });
                    inner_builder
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 2.), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 2.),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Visibility::Visible,
                            BorderColor(Color::BLACK),
                            BackgroundColor(Color::WHITE),
                            BorderRadius::all(Val::Px(8.0)),
                            UiInteractionBtn,
                            PaletteViewHideBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                ImageNode {
                                    image: asset_server.load("ui/view_120x120.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                                ViewHideImg,
                            ));
                        });
                    inner_builder
                        .spawn((
                            Button,
                            Node {
                                margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 2.), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 2.),
                                border: UiRect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::WHITE),
                            Visibility::Visible,
                            BorderColor(Color::BLACK),
                            BorderRadius::all(Val::Px(8.0)),
                            UiInteractionBtn,
                            PaletteTrashBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                ImageNode {
                                    image: asset_server.load("ui/trash2_60x60.png"),
                                    ..default()
                                },
                                Node {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                            ));
                        });
                });

            ///////////
            // COLOR VIEW ROW
            ///////////
            builder
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(2.0),
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderRadius::all(Val::Px(8.0)),
                    BackgroundColor(colors.node_color),
                ))
                .with_children(|inner_builder| {
                    // TEXT BOX

                    let mut rng = rand::thread_rng();
                    let random_color = default_color_palette
                        .colors
                        .choose(&mut rng)
                        .unwrap()
                        .to_srgba();
                    let random_color_string = random_color.to_hex();

                    inner_builder
                        .spawn((
                            Node {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                padding: UiRect {
                                    left: Val::Px(12.0),
                                    right: Val::Px(12.0),
                                    top: Val::Px(4.0),
                                    bottom: Val::Px(4.0),
                                },
                                //width: Val::Px(56.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(random_color.into()),
                            BorderRadius::all(Val::Px(8.0)),
                            ColorPaletteViewTextNode,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                Text::new(random_color_string),
                                TextFont {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: UI_SMALL_TEXT_SIZE,
                                    font_smoothing: FontSmoothing::AntiAliased,
                                },
                                TextColor(colors.text_color),
                                ColorPaletteViewText,
                            ));
                        });

                    // ADD TO PALETTE BTN FUTURE
                });

            ///////////
            // COLOR PALETTE ROW
            ///////////
            builder
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(4.0),
                        column_gap: Val::Px(4.0),
                        //width: Val::Auto,
                        //margin: UiRect::horizontal(Val::Px(4.0)),
                        flex_wrap: FlexWrap::Wrap,
                        padding: UiRect::vertical(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(colors.node_color),
                ))
                .with_children(|inner_builder| {
                    for color in &default_color_palette.colors {
                        inner_builder.spawn((
                            Button,
                            Node {
                                border: UiRect {
                                    left: Val::Px(2.0),
                                    right: Val::Px(0.0),
                                    top: Val::Px(2.0),
                                    bottom: Val::Px(0.0),
                                },
                                width: Val::Px(UI_ICON_SIZE / 2.5),
                                height: Val::Px(UI_ICON_SIZE / 2.5),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(*color),
                            BorderColor(DARK_GRAY.into()),
                            BorderRadius::all(Val::Px(3.0)),
                            UiInteractionBtn,
                            PaletteBtn,
                            IndividualColorInPalette(*color),
                        ));
                    }
                });
        });
        //     UiInteractionBtn,
        //     PaletteBtn,
        //     IndividualColorInPalette(*color),
        overall_inventory_node.set_parent(parent_node);
    }
}

pub fn show_layout(
    mut query: Query<&mut Node, With<PaintPaletteNode>>,

    mut inventory_state: ResMut<NextState<InventoryUiState>>,
    // mut inv_event: EventWriter<AddInventoryRow>,
) {
    for mut style in query.iter_mut() {
        inventory_state.set(InventoryUiState::Off);
        style.display = Display::Flex;
    }
}
pub fn highlight_pencil(
    mut query: Query<&mut BackgroundColor, With<PalettePencilBtn>>,
    colors: Res<ColorPalette>,
) {
    for mut bg_color in query.iter_mut() {
        *bg_color = BackgroundColor(colors.accent_color);
    }
}
pub fn hide_layout(
    mut query: Query<&mut Node, With<PaintPaletteNode>>,
    mut inventory_state: ResMut<NextState<InventoryUiState>>,
    mut movement_palette_state: ResMut<NextState<ToolPaletteUiState>>,
    mut move_btn_color_bg_q: Query<
        &mut BackgroundColor,
        (With<PaletteMoveBtn>, Without<PaintPaletteNode>),
    >,
    colors: Res<ColorPalette>,
) {
    for mut color in move_btn_color_bg_q.iter_mut() {
        *color = colors.light_color.into();
    }

    for mut style in query.iter_mut() {
        inventory_state.set(InventoryUiState::On);
        movement_palette_state.set(ToolPaletteUiState::Off);
        style.display = Display::None;
    }
}
