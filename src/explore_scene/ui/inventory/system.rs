use bevy::prelude::*;

use crate::{
    eventy::{DespawnInventoryHeights, TravelHeight},
    resourcey::{ColorPalette, UserInventoryBlocks},
};

use super::{
    component::{
        InventoryColorBox, InventoryColorBoxNode, InventoryHeightTextNode, InventoryNode,
        InventoryRowsNode, InventoryToggleButton, InventoryToggleable,
    },
    event::AddInventoryRow,
    layout::spawn_inventory_row,
};

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn inventory_adder_system(
    mut commands: Commands,
    mut inventory_row_node: Query<Entity, (With<InventoryRowsNode>, Without<InventoryColorBox>)>,
    mut inventory_node: Query<
        &mut Visibility,
        (
            With<InventoryNode>,
            Without<InventoryColorBox>,
            Without<InventoryRowsNode>,
        ),
    >,
    mut inventory_color_boxes: Query<
        (&mut BackgroundColor, &InventoryColorBox),
        (With<InventoryColorBox>, Without<InventoryRowsNode>),
    >,
    mut event: EventReader<AddInventoryRow>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,

    mut inventory: ResMut<UserInventoryBlocks>,
) {
    for inventory_event in event.read() {
        info!("inv event trig");

        let inventory_rows = inventory_row_node.get_single_mut().unwrap();
        let mut inventory_visibility = inventory_node.get_single_mut().unwrap();
        if *inventory_visibility == Visibility::Hidden {
            *inventory_visibility = Visibility::Visible;
        }
        for tile in &inventory_event.0 {
            let mut updated_block = false;
            // check to see if the player already owns the block, if so update background color of the square
            let check = inventory.ownedblocks.get(&tile.height);
            if check.is_some() {
                for (mut bg_color, comp_height) in inventory_color_boxes.iter_mut() {
                    if comp_height.0 == tile.height {
                        let new_color = Color::hex(tile.color.clone()).unwrap();
                        *bg_color = BackgroundColor(new_color);
                        updated_block = true;
                    }
                }
            }
            // add/update in new block to inventory. This should be done even if the owner upgrades, because we also update amount.
            inventory.ownedblocks.insert(tile.height, tile.clone());

            // if you already updated the block do not add the new block to the inventory.
            if !updated_block {
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                spawn_inventory_row(
                    &mut commands,
                    tile,
                    font.clone(),
                    colors.clone(),
                    inventory_rows,
                );
                //commands.entity(ent).add_child(child);
            }
        }
    }
}
//}

#[allow(clippy::type_complexity)]
pub fn inventory_remover_system(
    mut event: EventReader<DespawnInventoryHeights>,
    mut commands: Commands,
    mut inventory_height_node: Query<
        (Entity, &InventoryHeightTextNode),
        With<InventoryHeightTextNode>,
    >,
    mut inventory_colorbox_node: Query<
        (Entity, &InventoryColorBoxNode),
        (
            With<InventoryColorBoxNode>,
            Without<InventoryHeightTextNode>,
        ),
    >,
    mut inventory: ResMut<UserInventoryBlocks>,
) {
    for heights in event.read() {
        info!("despawn heights {:#?}", heights);

        for height in &heights.0 {
            for (ent, node) in inventory_height_node.iter_mut() {
                if &node.0 == height {
                    commands.entity(ent).despawn_recursive();
                }
            }
            for (ent, node) in inventory_colorbox_node.iter_mut() {
                if &node.0 == height {
                    commands.entity(ent).despawn_recursive();
                }
            }
            inventory.ownedblocks.remove(height);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn visible_inventory_toggle_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut UiImage),
        (Changed<Interaction>, With<InventoryToggleButton>),
    >,
    // mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
    mut inventory_rows_node_q: Query<
        &mut Style,
        (With<InventoryToggleable>, Without<InventoryToggleButton>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color, mut image) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                info!("inventory toggle");

                for mut style in inventory_rows_node_q.iter_mut() {
                    if style.display == Display::Flex || style.display == Display::Grid {
                        style.display = Display::None;

                        *image = UiImage {
                            texture: asset_server.load("ui/expandarrow_60x60.png"),
                            flip_x: false,
                            flip_y: true,
                        };
                    } else {
                        style.display = Display::Grid;
                        *image = UiImage::new(asset_server.load("ui/expandarrow_60x60.png"));
                    }
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.light_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn inventory_colorbox_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &InventoryColorBox),
        (Changed<Interaction>, With<InventoryColorBox>),
    >,
    colors: Res<ColorPalette>,
    mut travel: EventWriter<TravelHeight>,
) {
    for (interaction, mut color, color_box) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let height = color_box.0;
                *color = colors.accent_color.into();
                info!("inventory block clicked: {}", height);

                travel.send(TravelHeight(height));
            }
            Interaction::Hovered => {
                *color = colors.yellow_color.into();
            }
            Interaction::None => {
                *color = colors.light_color.into();
            }
        }
    }
}
