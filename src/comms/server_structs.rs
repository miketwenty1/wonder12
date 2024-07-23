use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockDataFromDB {
    pub height: i32,
    pub event_date: DateTime<Utc>,
    pub color: String,
    pub message: String,
    pub amount: i32,
    pub username: String,
    pub refund_ln_addr: String,
    pub hash: String,
    pub time: i64,
    pub bits: i64,
    pub n_tx: i32,
    pub size: i32,
    pub fee: i64,
    pub weight: i64,
    pub ver: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlockDataFromDBMod {
    pub blocks: Vec<GameBlockDataFromDB>,
    pub ts_checkpoint: Option<DateTime<Utc>>,
    pub height_checkpoint: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserGameBlock {
    pub height: u32,
    pub amount: u32,
    pub color: String,
}
