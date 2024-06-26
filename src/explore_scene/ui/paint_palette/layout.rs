use bevy::prelude::*;

use crate::{
    componenty::UiOverlayingExplorerButton,
    consty::{UI_ICON_SIZE, UI_MEDIUM_TEXT_SIZE},
    explore_scene::ui::{
        components::ExplorerUiNodeLeft,
        inventory::state::InventoryUiState,
        paint_palette::component::{
            AddToCustomPaletteBtn, ColorPaletteViewText, ColorPaletteViewTextNode, PaletteBtn,
            PaletteEraserBtn, PaletteEyedropBtn, PaletteMoveBtn, PaletteTrashBtn,
        },
    },
    resourcey::ColorPalette,
};

use super::{
    component::PaintPaletteNode,
    resource::{DefaultDrawColorPalette, MovementPaletteSelected},
    state::MovementPaletteUiState,
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
    info!("this being reached?");
    for parent_node in placement_query.iter() {
        let mut overall_inventory_node = commands.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    //height: Val::Percent(100.0),
                    // grid_template_columns: vec![GridTrack::auto()],
                    // grid_template_rows: vec![
                    //     GridTrack::min_content(),
                    //     GridTrack::min_content(),
                    //     GridTrack::min_content(),
                    // ],
                    // justify_items: JustifyItems::Start,
                    // justify_self: JustifySelf::Start,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::all(Val::Px(4.0)),
                    row_gap: Val::Px(6.0),
                    //width: Val::Px(80.0),
                    //height: Val::Percent(100.0),
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
                                border_color: BorderColor(Color::Rgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 1.0,
                                }),
                                background_color: BackgroundColor(Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                }),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            PaletteMoveBtn,
                            UiOverlayingExplorerButton,
                            PaletteBtn,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((ImageBundle {
                                style: Style {
                                    height: Val::Px(UI_ICON_SIZE / 1.7),
                                    width: Val::Px(UI_ICON_SIZE / 1.7),
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
                                border_color: BorderColor(Color::Rgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 1.0,
                                }),
                                background_color: BackgroundColor(Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                }),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            PaletteEraserBtn,
                            UiOverlayingExplorerButton,
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
                                border_color: BorderColor(Color::Rgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 1.0,
                                }),
                                background_color: BackgroundColor(Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                }),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            PaletteEyedropBtn,
                            UiOverlayingExplorerButton,
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
                                border_color: BorderColor(Color::Rgba {
                                    red: 0.0,
                                    green: 0.0,
                                    blue: 0.0,
                                    alpha: 1.0,
                                }),
                                background_color: BackgroundColor(Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                }),
                                visibility: Visibility::Visible,
                                ..default()
                            },
                            PaletteTrashBtn,
                            UiOverlayingExplorerButton,
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
                        column_gap: Val::Px(6.0),
                        padding: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                })
                .with_children(|inner_builder| {
                    // TEXT BOX
                    inner_builder
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    padding: UiRect::all(Val::Px(4.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: BackgroundColor(Color::ORANGE_RED),
                                ..default()
                            },
                            ColorPaletteViewTextNode,
                        ))
                        .with_children(|in_in_builder| {
                            in_in_builder.spawn((
                                TextBundle::from_section(
                                    "#FF0000",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: UI_MEDIUM_TEXT_SIZE,
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
                                padding: UiRect::all(Val::Px(4.0)),
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
                                            margin: UiRect::all(Val::Px(3.0)),
                                            width: Val::Px(UI_ICON_SIZE), // to make it a square.
                                            height: Val::Px(UI_ICON_SIZE / 1.5),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        border_color: BorderColor(Color::Rgba {
                                            red: 0.0,
                                            green: 0.0,
                                            blue: 0.0,
                                            alpha: 1.0,
                                        }),
                                        background_color: BackgroundColor(Color::Rgba {
                                            red: 0.0,
                                            green: 0.0,
                                            blue: 1.0,
                                            alpha: 1.0,
                                        }),
                                        visibility: Visibility::Visible,
                                        ..default()
                                    },
                                    AddToCustomPaletteBtn,
                                ))
                                .with_children(|in_in_in_b| {
                                    in_in_in_b.spawn((
                                        TextBundle::from_section(
                                            "Add to\nPalette".to_string(),
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: UI_MEDIUM_TEXT_SIZE,
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
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BEIGE),
                    ..default()
                })
                .with_children(|inner_builder| {
                    for color in &default_color_palette.colors {
                        inner_builder.spawn(ButtonBundle {
                            style: Style {
                                //margin: UiRect::all(Val::Px(3.0)),
                                width: Val::Px(UI_ICON_SIZE / 2.), // to make it a square.
                                height: Val::Px(UI_ICON_SIZE / 2.),
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(*color),
                            visibility: Visibility::Visible,
                            ..default()
                        });
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
pub fn hide_layout(
    mut query: Query<&mut Style, With<PaintPaletteNode>>,
    mut inventory_state: ResMut<NextState<InventoryUiState>>,
    mut movement_palette_state: ResMut<NextState<MovementPaletteUiState>>,
    mut selected: ResMut<MovementPaletteSelected>,
    mut move_btn_color_bg_q: Query<
        &mut BackgroundColor,
        (With<PaletteMoveBtn>, Without<PaintPaletteNode>),
    >,
    colors: Res<ColorPalette>,
) {
    for mut color in move_btn_color_bg_q.iter_mut() {
        *color = colors.light_color.into();
    }
    *selected = MovementPaletteSelected(false);

    for mut style in query.iter_mut() {
        inventory_state.set(InventoryUiState::On);
        movement_palette_state.set(MovementPaletteUiState::Off);
        style.display = Display::None;
    }
}
