use bevy::prelude::*;
use rand::Rng;

use crate::{
    eventy::{UpdateTileTextureEvent, UpdateTilesAfterPurchase},
    resourcey::{TileCartVec, TileData, WorldOwnedTileMap},
    structy::TileResource,
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
        info!("received event now will update map!!!!!!!");
        for tile in &tile_cart_vec.vec {
            let new_td = TileData {
                ln_address: tile.new_ln_address.to_string(),
                username: tile.username.to_string(),
                color: tile.new_color,
                message: tile.new_message.to_string(),
                resource: TileResource::Wheat,
                value: tile.cost,
                cost: (tile.cost * 2),
                hash: "".to_string(),
                height: tile.height,
                land_index: rng.gen_range(1..=11),
                event_date: tile.event_date.unwrap_or_default(),
            };
            new_tile_vec.push(new_td.clone());
            tile_map.map.insert(new_td.height, new_td);
        }

        update_tile_event.send(UpdateTileTextureEvent(new_tile_vec));
    }
}
