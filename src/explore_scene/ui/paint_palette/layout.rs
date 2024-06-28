use bevy::{prelude::*, ui::FocusPolicy};
use rand::seq::SliceRandom;
use rand::Rng;

use crate::{
    componenty::{UiInteractionBtn, UiOverlayingExplorerButton},
    consty::{UI_ICON_SIZE, UI_MEDIUM_TEXT_SIZE, UI_SMALL_TEXT_SIZE},
    explore_scene::ui::{
        components::ExplorerUiNodeLeft,
        inventory::state::InventoryUiState,
        paint_palette::component::{
            AddToCustomPaletteBtn, ColorPaletteViewText, ColorPaletteViewTextNode, PaletteBtn,
            PaletteEraserBtn, PaletteEyedropBtn, PaletteMoveBtn, PalettePencilBtn, PaletteTrashBtn,
        },
    },
    resourcey::ColorPalette,
    utils::convert_color_to_hexstring,
};

use super::{
    component::{IndividualColorInPalette, PaintPaletteNode},
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
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::all(Val::Px(4.0)),
                    row_gap: Val::Px(6.0),
                    ..default()
                },
                visibility: Visibility::Visible,
                background_color: BackgroundColor(Color::BLUE), //colors.node_color),
                ..default()
            },
            PaintPaletteNode,
        ));

        overall_inventory_node.with_children(|builder| {
            ///////////
            // MOVE ROW
            ///////////
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(6.0),
                        column_gap: Val::Px(8.0),
                        padding: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                })
                .with_children(|inner_builder| {
                    inner_builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
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
                                //image: UiImage::new(asset_server.load("ui/palette_120x120.png")),
                                border_color: BorderColor(Color::BLACK),
                                background_color: BackgroundColor(colors.accent_color),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            UiInteractionBtn,
                            PalettePencilBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((ImageBundle {
                                style: Style {
                                    height: Val::Px(UI_ICON_SIZE / 2.),
                                    width: Val::Px(UI_ICON_SIZE / 2.),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("ui/pencil_120x120.png")),
                                ..default()
                            },));
                        });

                    inner_builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
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
                                //image: UiImage::new(asset_server.load("ui/palette_120x120.png")),
                                border_color: BorderColor(Color::BLACK),
                                background_color: BackgroundColor(Color::WHITE),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            UiInteractionBtn,
                            PaletteMoveBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((ImageBundle {
                                style: Style {
                                    height: Val::Px(UI_ICON_SIZE / 2.),
                                    width: Val::Px(UI_ICON_SIZE / 2.),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("ui/move_60x60.png")),
                                ..default()
                            },));
                        });
                });

            ///////////
            // TOOLS ROW
            ///////////
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(6.0),
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                })
                .with_children(|inner_builder| {
                    inner_builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
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
                                //image: UiImage::new(asset_server.load("ui/palette_120x120.png")),
                                border_color: BorderColor(Color::BLACK),
                                background_color: BackgroundColor(Color::WHITE),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            UiInteractionBtn,
                            PaletteEraserBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((ImageBundle {
                                style: Style {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("ui/eraser_60x60.png")),
                                ..default()
                            },));
                        });
                    inner_builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
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
                                //image: UiImage::new(asset_server.load("ui/palette_120x120.png")),
                                border_color: BorderColor(Color::BLACK),
                                background_color: BackgroundColor(Color::WHITE),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            UiInteractionBtn,
                            PaletteEyedropBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((ImageBundle {
                                style: Style {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("ui/eyedrop_60x60.png")),
                                ..default()
                            },));
                        });
                    inner_builder
                        .spawn((
                            ButtonBundle {
                                style: Style {
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
                                //image: UiImage::new(asset_server.load("ui/palette_120x120.png")),
                                border_color: BorderColor(Color::BLACK),
                                background_color: BackgroundColor(Color::WHITE),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            UiInteractionBtn,
                            PaletteTrashBtn,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((ImageBundle {
                                style: Style {
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    width: Val::Px(UI_ICON_SIZE / 2.5),
                                    ..default()
                                },
                                image: UiImage::new(asset_server.load("ui/trash2_60x60.png")),
                                ..default()
                            },));
                        });
                });

            ///////////
            // COLOR VIEW ROW
            ///////////
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(2.0),
                        padding: UiRect::all(Val::Px(2.0)),

                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                })
                .with_children(|inner_builder| {
                    // TEXT BOX

                    let mut rng = rand::thread_rng();
                    let random_color = default_color_palette.colors.choose(&mut rng).unwrap();
                    let random_color_string = convert_color_to_hexstring(*random_color);

                    inner_builder
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    padding: UiRect::all(Val::Px(4.0)),
                                    width: Val::Px(56.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(*random_color),
                                ..default()
                            },
                            ColorPaletteViewTextNode,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                TextBundle::from_section(
                                    format!("#{}", random_color_string),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: UI_SMALL_TEXT_SIZE,
                                        color: colors.text_color,
                                    },
                                ),
                                ColorPaletteViewText,
                            ));
                        });

                    // ADD TO PALETTE BTN
                    inner_builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                row_gap: Val::Px(6.0),
                                //padding: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::ORANGE_RED),
                            ..default()
                        })
                        .with_children(|in_in_builder| {
                            in_in_builder
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            //margin: UiRect::all(Val::Px(0.0)),
                                            width: Val::Px(UI_ICON_SIZE), // to make it a square.
                                            height: Val::Px(UI_ICON_SIZE / 1.5),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::BLACK),
                                        background_color: BackgroundColor(Color::BLUE),
                                        visibility: Visibility::Visible,
                                        ..default()
                                    },
                                    UiInteractionBtn,
                                    PaletteBtn,
                                    AddToCustomPaletteBtn,
                                ))
                                .with_children(|in_in_in_b| {
                                    in_in_in_b.spawn((
                                        TextBundle::from_section(
                                            "Add to\nPalette".to_string(),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: UI_SMALL_TEXT_SIZE,
                                                color: colors.text_color,
                                            },
                                        ),
                                        ColorPaletteViewText,
                                    ));
                                });
                        });
                });

            ///////////
            // COLOR PALETTE ROW
            ///////////
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(4.0),
                        column_gap: Val::Px(4.0),
                        width: Val::Auto,
                        flex_wrap: FlexWrap::Wrap,
                        padding: UiRect::vertical(Val::Px(4.0)),
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::rgb(0.75, 0.70, 0.80)),
                    ..default()
                })
                .with_children(|inner_builder| {
                    for color in &default_color_palette.colors {
                        inner_builder.spawn((
                            ButtonBundle {
                                style: Style {
                                    border: UiRect {
                                        left: Val::Px(2.0),
                                        right: Val::Px(0.0),
                                        top: Val::Px(2.0),
                                        bottom: Val::Px(0.0),
                                    },
                                    //margin: UiRect::all(Val::Px(3.0)),
                                    width: Val::Px(UI_ICON_SIZE / 2.5), // to make it a square.
                                    height: Val::Px(UI_ICON_SIZE / 2.5),
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_color: BorderColor(Color::DARK_GRAY),
                                background_color: BackgroundColor(*color),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            UiInteractionBtn,
                            PaletteBtn,
                            IndividualColorInPalette(*color),
                        ));
                    }
                });
        });

        overall_inventory_node.set_parent(parent_node);
    }
}

pub fn show_layout(
    mut query: Query<&mut Style, With<PaintPaletteNode>>,

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
    mut query: Query<&mut Style, With<PaintPaletteNode>>,
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
