use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlocksDataFromDB {
    pub height: i32,
    pub event_date: DateTime<Utc>,
    pub color: String,
    pub message: String,
    pub amount: i32,
    pub username: String,
    pub refund_ln_addr: String,
    pub block_hash: String,
    pub block_time: i64,
    pub block_bits: i64,
    pub block_n_tx: i32,
    pub block_size: i32,
    pub block_fee: i64,
    pub block_weight: i64,
    pub block_ver: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlocksDataFromDBMod {
    pub blocks: Vec<GameBlocksDataFromDB>,
    pub ts_checkpoint: Option<DateTime<Utc>>,
    pub height_checkpoint: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserGameBlock {
    pub height: u32,
    pub amount: u32,
    pub color: String,
}
