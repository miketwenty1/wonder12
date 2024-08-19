use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comms::server_structs::UserGameBlock;

// #[derive(Debug)]
// pub enum EdgeType {
//     Top,
//     Bottom,
//     Left,
//     Right,
// }

#[derive(Clone)]
pub struct EdgeData {
    pub tile: i32,
    pub pixel: f32,
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileResource {
    Mountain,
    Water,
    Grass,
    Forest,
    Desert,
    #[default]
    Unknown,
}
impl TileResource {
    pub fn spritesheet_index_value(&self) -> usize {
        match self {
            TileResource::Mountain => 0,
            TileResource::Water => 1,
            TileResource::Grass => 2,
            TileResource::Forest => 3,
            TileResource::Desert => 4,
            TileResource::Unknown => 35,
        }
    }
}

#[derive(Debug)]
pub struct SpawnDiffData {
    pub xstart: i32,
    pub xend: i32,
    pub ystart: i32,
    pub yend: i32,
}

#[derive(Clone, Debug)]
pub enum TileTextType {
    Height,
    Value,
    Blank,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InvoiceGameBlock {
    pub height: u32,
    pub color: String,
    pub message: String,
    pub amount: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameInvoiceData {
    pub blocks: Vec<InvoiceGameBlock>,
    pub username: String,
    pub refund_address: String,
}

#[derive(Clone, Debug)]
pub enum RequestTileType {
    Height,
    Ts,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ErrorMessage {
    pub error: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserInventoryBlocksFromServer {
    pub ownedblocks: Vec<UserGameBlock>,
}

impl UserInventoryBlocksFromServer {
    pub fn map(&self) -> HashMap<u32, UserGameBlock> {
        self.ownedblocks
            .iter()
            .map(|block| (block.height, block.clone()))
            .collect()
    }
}
