use bevy::prelude::*;
use rand::Rng;

use crate::{
    consty::{DEFAULT_HASH, INDEX_MAX_LAND, INDEX_WHITE_LAND},
    eventy::{UpdateTileTextureEvent, UpdateTilesAfterPurchase},
    resourcey::{TileCartVec, TileData, WorldOwnedTileMap},
    structy::TileResource,
    utils::{get_land_index, get_resource_for_tile},
};

#[allow(clippy::too_many_arguments)]
pub fn update_tiles_after_purchase(
    mut event: EventReader<UpdateTilesAfterPurchase>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
    tile_cart_vec: Res<TileCartVec>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
) {
    for _e in event.read() {
        let mut new_tile_vec = Vec::new();
        let mut rng = rand::thread_rng();
        info!("updating after purchase!");

        for tile in &tile_cart_vec.vec {
            let tile_data_check = tile_map.map.get(&tile.height);
            let new_td = match tile_data_check {
                Some(s) => {
                    let resource = get_resource_for_tile(&s.block_hash);
                    let land_index = get_land_index(tile.height, &resource, None);

                    TileData {
                        ln_address: tile.new_ln_address.to_string(),
                        username: tile.username.to_string(),
                        color: tile.new_color,
                        message: tile.new_message.to_string(),
                        value: tile.cost,
                        cost: (tile.cost * 2),
                        event_date: tile.event_date.unwrap_or_default(),
                        land_index: land_index,
                        resource: resource,
                        height: tile.height,
                        block_hash: s.block_hash.clone(),
                        block_time: s.block_time,
                        block_bits: s.block_bits,
                        block_n_tx: s.block_n_tx,
                        block_size: s.block_size,
                        block_fee: s.block_fee,
                        block_weight: s.block_weight,
                        block_ver: s.block_ver,
                    }
                }
                None => TileData {
                    ln_address: tile.new_ln_address.to_string(),
                    username: tile.username.to_string(),
                    color: tile.new_color,
                    message: tile.new_message.to_string(),
                    value: tile.cost,
                    cost: (tile.cost * 2),
                    event_date: tile.event_date.unwrap_or_default(),
                    land_index: INDEX_WHITE_LAND,
                    resource: TileResource::Desert,
                    height: tile.height,
                    block_hash: "0000000000000000000000000000000000000000000000000000000000000000"
                        .to_string(),
                    block_time: 0,
                    block_bits: 0,
                    block_n_tx: 0,
                    block_size: 0,
                    block_fee: 0,
                    block_weight: 0,
                    block_ver: 0,
                },
            };

            new_tile_vec.push(new_td.clone());
            tile_map.map.insert(new_td.height, new_td);
        }

        update_tile_event.send(UpdateTileTextureEvent(new_tile_vec));
    }
}
