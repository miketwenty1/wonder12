use bevy::{render::color::LegacyColor, utils::HashMap};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    resourcey::{TileData, WorldOwnedTileMap},
    utils::derive_cost_from_value,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimTileLocalBrowserStorage {
    pub map: HashMap<u32, TrimTile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimTile {
    // color
    pub c: String,
    // value
    pub v: u32,
    // hash
    pub h: String,
    // lightning address
    pub l: String,
    // message
    pub m: String,
    // username
    pub u: String,
    // date
    pub d: DateTime<Utc>,
}

impl TrimTileLocalBrowserStorage {
    pub fn convert_trim_to_tilemap(self) -> WorldOwnedTileMap {
        let mut tile_map = HashMap::new();
        let mut rng = rand::thread_rng();
        for (key, trim_tile) in self.map.into_iter() {
            let tile_data = TileData {
                ln_address: trim_tile.l,
                username: trim_tile.u,
                color: LegacyColor::hex(&trim_tile.c).unwrap(),
                message: trim_tile.m,
                resource: crate::structy::TileResource::Wheat,
                hash: trim_tile.h,
                value: trim_tile.v,
                cost: derive_cost_from_value(trim_tile.v),
                height: key,
                land_index: rng.gen_range(1..=11),
                event_date: trim_tile.d,
            };
            tile_map.insert(key, tile_data);
        }

        WorldOwnedTileMap { map: tile_map }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockMessagesFromDB {
    pub username: String,
    pub message: String,
    pub amount: i32,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MessagesFromServer {
    pub height: u32,
    pub messages: Vec<GameBlockMessagesFromDB>
}