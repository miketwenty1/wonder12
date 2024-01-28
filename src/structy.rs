use bevy::utils::hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::comms::server_structs::UserGameBlock;

#[derive(Debug)]
pub enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone)]
pub struct EdgeData {
    pub tile: i32,
    pub pixel: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileResource {
    Wheat,
    // Brick,
    // Sheep,
    // Wood,
    // Stone,
    // Desert,
    // Water,
    // Grass,
    // Unknown,
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
