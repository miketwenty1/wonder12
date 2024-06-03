use bevy::prelude::*;

use crate::{
    comms::server_structs::UserGameBlock,
    componenty::UiOverlayingExplorerButton,
    consty::UI_MEDIUM_TEXT_SIZE,
    explore_scene::ui::{
        components::ExplorerUiNodeLeft,
        inventory::component::{InventoryRowsNode, InventoryToggleButton, InventoryToggleable},
    },
    resourcey::{ColorPalette, UserInventoryBlocks},
};

use super::{
    component::{
        InventoryColorBox, InventoryColorBoxNode, InventoryHeightText, InventoryHeightTextNode,
        InventoryNode,
    },
    event::AddInventoryRow,
};

pub fn spawn_layout(
    mut commands: Commands,
    inventory_blocks: Res<UserInventoryBlocks>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeLeft>>,
    mut inv_event: EventWriter<AddInventoryRow>,
) {
    for parent_node in placement_query.iter() {
        let mut should_add_init_inventory = false;
        let visibility = if inventory_blocks.ownedblocks.is_empty() {
            Visibility::Hidden
        } else {
            // in the visibility logic also add in the blocks that are already owned.

            should_add_init_inventory = true;
            Visibility::Visible
        };

        info!("spawn inventory layout");
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
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
                    justify_items: JustifyItems::Start,
                    justify_self: JustifySelf::Start,
                    flex_direction: FlexDirection::Column,
                    // padding: UiRect::all(Val::Px(4.0)),
                    margin: UiRect::bottom(Val::Auto),

                    ..default()
                },
                visibility,
                background_color: BackgroundColor(colors.node_color_lighter), //colors.node_color),
                ..default()
            },
            InventoryNode,
        ));

        // inventory text row
        overall_inventory_node.with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        // grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        // grid_template_rows: vec![GridTrack::auto()],
                        flex_direction: FlexDirection::Row,
                        row_gap: Val::Px(6.0),
                        padding: UiRect::all(Val::Px(4.0)),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                })
                .with_children(|child_builder| {
                    // inventory text
                    child_builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect::all(Val::Px(4.0)),
                                margin: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(colors.node_color),
                            ..default()
                        })
                        .with_children(|inner| {
                            inner.spawn(TextBundle::from_section(
                                "Inventory",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: UI_MEDIUM_TEXT_SIZE,
                                    color: colors.text_color,
                                },
                            ));
                        });
                    //where button will live
                    child_builder
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                padding: UiRect::all(Val::Px(4.0)),
                                margin: UiRect::all(Val::Px(4.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(colors.node_color),
                            ..default()
                        })
                        .with_children(|inner| {
                            inner.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(20.0),
                                        height: Val::Px(20.0),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        border: UiRect::all(Val::Px(2.0)),
                                        ..default()
                                    },
                                    border_color: BorderColor(colors.light_color),
                                    background_color: colors.light_color.into(),
                                    image: UiImage::new(
                                        asset_server.load("ui/expandarrow60x60.png"),
                                    ),
                                    ..default()
                                },
                                InventoryToggleButton,
                                UiOverlayingExplorerButton,
                            ));
                        });
                });
        });

        // inventory blocks row
        overall_inventory_node.with_children(|builder| {
            builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Start,
                        justify_self: JustifySelf::Start,
                        grid_template_columns: vec![
                            GridTrack::min_content(),
                            GridTrack::min_content(),
                        ],
                        //grid_template_rows: vec![GridTrack::min_content()],
                        grid_auto_flow: GridAutoFlow::Row,
                        padding: UiRect::all(Val::Px(4.0)),
                        margin: UiRect::all(Val::Px(4.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                },
                InventoryRowsNode,
                InventoryToggleable,
            ));

            //finally set inventory
            if should_add_init_inventory {
                let mut inv: Vec<UserGameBlock> = Vec::new();
                for (_height, user_game_block) in &inventory_blocks.ownedblocks {
                    inv.push(user_game_block.clone());
                }
                inv.sort_by(|a, b| b.amount.cmp(&a.amount));
                inv_event.send(AddInventoryRow(inv));
            }
        });

        // placeholder row in the grid to help with looks
        // overall_inventory_node.with_children(|builder| {
        //     builder.spawn((
        //         NodeBundle {
        //             style: Style {
        //                 display: Display::Grid,
        //                 //grid_template_rows: vec![GridTrack::min_content()],
        //                 grid_auto_flow: GridAutoFlow::Row,
        //                 ..default()
        //             },
        //             background_color: BackgroundColor(Color::GOLD),
        //             ..default()
        //         },
        //         PlaceHolderInventoryNode,
        //         InventoryToggleable,
        //     ));
        // });
        overall_inventory_node.set_parent(parent_node);
    }
}

pub fn spawn_inventory_row(
    builder: &mut Commands,
    block: &UserGameBlock,
    font: Handle<Font>,
    colors: ColorPalette,
    inventory_rows_node: Entity,
) {
    let _left_item = builder
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    //width: Val::Px(200.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    // align_content: AlignContent::SpaceAround,
                    // justify_items: JustifyItems::Start,
                    height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(3.0)),
                    margin: UiRect::all(Val::Px(3.0)),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            InventoryHeightTextNode(block.height),
        ))
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    format!("{}", block.height),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: colors.text_color,
                    },
                ),
                InventoryHeightText(block.height),
            ));
        })
        .set_parent(inventory_rows_node);

    let _right_item = builder
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    //width: Val::Px(200.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    // align_content: AlignContent::SpaceAround,
                    // justify_items: JustifyItems::Start,
                    height: Val::Px(30.0),
                    padding: UiRect::all(Val::Px(3.0)),
                    margin: UiRect::all(Val::Px(3.0)),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            InventoryColorBoxNode(block.height),
        ))
        .with_children(|builder| {
            builder.spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(26.0),
                        height: Val::Px(26.0),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.light_color),
                    background_color: Color::hex(block.color.clone()).unwrap().into(),
                    ..default()
                },
                InventoryColorBox(block.height),
                UiOverlayingExplorerButton,
            ));
        })
        .set_parent(inventory_rows_node);
}
