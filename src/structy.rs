use serde::{Deserialize, Serialize};

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

#[derive(Clone)]
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
