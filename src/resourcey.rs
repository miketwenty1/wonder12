use bevy::{prelude::*, utils::HashMap};

use crate::structy::EdgeData;

use chrono::{DateTime, Utc};

#[derive(Resource, Clone, PartialEq, Debug)]
pub struct TileData {
    pub ln_address: String,
    pub username: String,
    pub color: Color,
    pub message: String,
    pub resource: i32,
    pub hash: String,
    pub value: u32,
    pub cost: u32,
    pub height: u32,
    pub land_index: usize,
    pub event_date: DateTime<Utc>,
}

#[derive(Resource, Clone, PartialEq)]
pub struct WorldOwnedTileMap {
    pub map: HashMap<u32, TileData>,
}

#[derive(Resource, Clone, Debug)]
pub struct TileCartData {
    pub event_date: Option<DateTime<Utc>>,
    pub ln_address: String,
    pub username: String,
    pub color: Option<Color>,
    pub message: String,
    pub value: u32,
    pub cost: u32,
    pub height: u32,
    pub new_ln_address: String,
    pub new_username: String,
    pub new_color: Color,
    pub new_color_text: String,
    pub new_message: String,
}

#[derive(Resource, Clone)]
pub struct TileCart {
    pub map: HashMap<u32, TileCartData>,
}

#[derive(Resource, Clone)]
pub struct TileCartVec {
    pub vec: Vec<TileCartData>,
    pub index: usize,
}

#[derive(Resource, Clone)]
pub struct ChunkManager {
    pub map: HashMap<u32, bool>,
}

#[derive(Resource, Clone)]
pub struct Edge {
    pub top: EdgeData,
    pub bottom: EdgeData,
    pub left: EdgeData,
    pub right: EdgeData,
}

#[derive(Resource, Clone)]
pub struct SpriteSheetBg {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}
