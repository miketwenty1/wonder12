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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameBlocksDataFromDBMod {
    pub blocks: Vec<GameBlocksDataFromDB>,
    pub ts_checkpoint: Option<DateTime<Utc>>,
    pub height_checkpoint: Option<u32>,
}
