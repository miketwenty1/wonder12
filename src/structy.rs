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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TileResource {
    Mountain,
    Water,
    Grass,
    Forest,
    Desert,
    Unknown,
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

struct Hex64([u8; 32]);

impl Hex64 {
    // Function to create a Hex64 from a 64-character hex string
    fn from_hex_string(hex: &str) -> Result<Self, std::num::ParseIntError> {
        assert!(hex.len() == 64, "Hex string must be 64 characters long");
        let mut bytes = [0u8; 32];
        for (i, byte) in bytes.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16)?;
        }
        Ok(Hex64(bytes))
    }

    // Function to process the last hex character as per the given rules
    fn process_last_hex_char(&self) -> u8 {
        let last_byte = self.0[31];
        let last_char = last_byte & 0x0F; // Get the last hex character (0-15)
        match last_char {
            0..=4 => last_char,
            _ => 2, // All values from 5 to F result in 2
        }
    }
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
