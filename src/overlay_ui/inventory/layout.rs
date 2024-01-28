use bevy::prelude::*;

use crate::{
    comms::server_structs::UserGameBlock,
    overlay_ui::inventory::component::{InnerInventoryNode, InventoryColorBox},
    resourcey::{ColorPalette, UserInventoryBlocks},
};

use super::component::{InventoryHeight, InventoryNode};

pub fn spawn_layout(
    mut commands: Commands,
    inventory_blocks: Res<UserInventoryBlocks>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
) {
    let visibility = if inventory_blocks.ownedblocks.is_empty() {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
    info!("spawn inventory layout");
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let mut parent = commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                height: Val::Percent(100.0),
                //justify_content: JustifyContent::Center,
                //width: Val::Px(200.0),
                //align_items: AlignItems::Start,
                //justify_content: JustifyContent::Start,
                //align_content: AlignContent::Start,
                //justify_items: JustifyItems::Start,
                padding: UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(4.0),
                    top: Val::Px(60.0),
                    bottom: Val::Px(10.0),
                },

                flex_direction: FlexDirection::Column,
                ..default()
            },
            visibility,
            background_color: BackgroundColor(colors.node_color),
            ..default()
        },
        InventoryNode,
    ));

    parent.with_children(|builder| {
        builder.spawn(TextBundle::from_section(
            "Inventory",
            TextStyle {
                font: font.clone(),
                font_size: 24.0,
                color: colors.text_color,
            },
        ));
    });

    parent.with_children(|builder| {
        for block in inventory_blocks.ownedblocks.values() {
            spawn_inventory_row(builder, block, font.clone(), colors.clone());
        }
    });
}

pub fn spawn_inventory_row(
    builder: &mut ChildBuilder,
    block: &UserGameBlock,
    font: Handle<Font>,
    colors: ColorPalette,
) {
    let mut row = builder.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                //width: Val::Px(200.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // align_content: AlignContent::SpaceAround,
                // justify_items: JustifyItems::Start,
                padding: UiRect::all(Val::Px(3.0)),
                margin: UiRect::all(Val::Px(3.0)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        InnerInventoryNode(block.height),
    ));

    row.with_children(|childrow| {
        childrow.spawn(TextBundle::from_section(
            format!("{}", block.height),
            TextStyle {
                font: font.clone(),
                font_size: 20.0,
                color: colors.text_color,
            },
        ));
    });

    row.with_children(|childrow| {
        childrow.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(6.0)),
                    width: Val::Px(26.0),
                    height: Val::Px(26.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                border_color: BorderColor(Color::WHITE),
                background_color: BackgroundColor(Color::hex(&block.color).unwrap()), //node_color
                ..default()
            },
            InventoryColorBox,
        ));
    });
}

pub fn spawn_inventory_row_c(
    builder: &mut Commands,
    block: &UserGameBlock,
    font: Handle<Font>,
    colors: ColorPalette,
) -> Entity {
    let mut row = builder.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                //width: Val::Px(200.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // align_content: AlignContent::SpaceAround,
                // justify_items: JustifyItems::Start,
                padding: UiRect::all(Val::Px(3.0)),
                margin: UiRect::all(Val::Px(3.0)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        InnerInventoryNode(block.height),
    ));

    row.with_children(|childrow| {
        childrow.spawn((
            TextBundle::from_section(
                format!("{}", block.height),
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: colors.text_color,
                },
            ),
            InventoryHeight,
        ));
    });

    row.with_children(|childrow| {
        childrow.spawn((
            ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(6.0)),
                    width: Val::Px(26.0),
                    height: Val::Px(26.0),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                border_color: BorderColor(Color::WHITE),
                background_color: BackgroundColor(Color::hex(&block.color).unwrap()), //node_color
                ..default()
            },
            InventoryColorBox,
        ));
    });
    row.id()
}
