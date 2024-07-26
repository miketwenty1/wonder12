use bevy::{color::Srgba, utils::HashMap};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::{
    consty::INDEX_MAX_LAND,
    resourcey::{TileData, WorldOwnedTileMap},
    utils::{derive_cost_from_value, get_land_index, get_resource_for_tile},
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
    // lightning address
    pub l: String,
    // message
    pub m: String,
    // username
    pub u: String,
    // date
    pub d: DateTime<Utc>,
    // hash
    pub h: String,
    // block time
    pub bt: i64,
    // block time
    pub bb: i64,
    // block number of transactions
    pub bn: i32,
    // block size
    pub bs: i32,
    // block fee
    pub bf: i64,
    // block weight
    pub bw: i64,
    // block version
    pub bv: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimExplorerTileVec(pub Vec<TrimExplorerTile>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrimExplorerTile {
    pub h: i32,    // height
    pub x: String, // hash
    pub t: i64,    // time
    pub b: i64,    // bits
    pub n: i32,    // n_tx (number of transactions)
    pub s: i32,    // size
    pub f: i64,    // fee
    pub w: i64,    // weight
    pub v: i32,    // ver (version)
}

impl TrimTileLocalBrowserStorage {
    pub fn convert_trim_to_tilemap(self) -> WorldOwnedTileMap {
        let mut tile_map = HashMap::new();
        //let mut rng = rand::thread_rng();
        for (key, trim_tile) in self.map.into_iter() {
            let resource = get_resource_for_tile(&trim_tile.h);
            let land_index = get_land_index(key, &resource, None);
            let tile_data = TileData {
                ln_address: trim_tile.l,
                username: trim_tile.u,
                color: Srgba::hex(&trim_tile.c).unwrap().into(),
                message: trim_tile.m,
                value: trim_tile.v,
                cost: derive_cost_from_value(trim_tile.v),
                height: key,
                land_index,
                event_date: trim_tile.d,
                resource,
                block_hash: trim_tile.h,
                block_time: trim_tile.bt,
                block_bits: trim_tile.bb,
                block_n_tx: trim_tile.bn,
                block_size: trim_tile.bs,
                block_fee: trim_tile.bf,
                block_weight: trim_tile.bw,
                block_ver: trim_tile.bv,
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
    pub messages: Vec<GameBlockMessagesFromDB>,
}
