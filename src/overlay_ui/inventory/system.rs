use bevy::prelude::*;

use crate::{
    comms::server_structs::UserGameBlock,
    resourcey::{ColorPalette, TileCartVec},
    utils::convert_color_to_hexstring,
};

use super::{
    component::{InnerInventoryNode, InventoryNode},
    event::AddInventoryRow,
    layout::spawn_inventory_row_c,
};

pub fn inventory_adder_system(
    mut commands: Commands,
    mut inventory_ent: Query<(Entity, &mut Visibility), With<InventoryNode>>,
    mut event: EventReader<AddInventoryRow>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    tile_cart_vec: Res<TileCartVec>,
    mut inner_inv: Query<(Entity, &InnerInventoryNode)>,
) {
    for _e in event.read() {
        info!("inv event trig");
        for (ent, mut visi) in inventory_ent.iter_mut() {
            info!("entity found");
            *visi = Visibility::Visible;
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            for tile in &tile_cart_vec.vec {
                let user_game_block = UserGameBlock {
                    height: tile.height,
                    amount: tile.value,
                    color: convert_color_to_hexstring(tile.new_color),
                };

                // despawn old if exist
                for (ent, node) in inner_inv.iter_mut() {
                    if node.0 == user_game_block.height {
                        commands.entity(ent).despawn_recursive();
                    }
                }

                // spawn new inv row
                let child = spawn_inventory_row_c(
                    &mut commands,
                    &user_game_block,
                    font.clone(),
                    colors.clone(),
                );

                commands.entity(ent).add_child(child);
            }
        }
    }
}

pub fn inventory_interaction() {}
