use async_channel::{Receiver, Sender};
use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

use crate::structy::EdgeData;
use crate::structy::TileResource;
use chrono::{DateTime, Utc};

#[derive(Resource, Clone)]
pub struct TileData {
    pub ln_address: String,
    pub username: String,
    pub color: Color,
    pub message: String,
    pub resource: TileResource,
    pub hash: String,
    pub value: u32,
    pub cost: u32,
    pub height: u32,
    pub land_index: usize,
    pub event_date: DateTime<Utc>,
}

#[derive(Resource, Clone)]
pub struct TileMap {
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
pub struct SpriteIndexBuilding(pub HashMap<u32, u32>);

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

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct SpriteSheetBuildingRes(pub Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct SpriteSheetBgRes(pub Handle<TextureAtlas>);

#[derive(Resource, Clone, Copy)]
pub struct LastSelectedTile(pub i32, pub i32);

#[derive(Resource, Clone)]
pub struct TileDataChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct RequestInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct CheckInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct ServerURL(pub String);

#[derive(Resource, Clone)]
pub struct User {
    pub ln_address: String,
    pub name: String,
}

#[derive(Resource, Clone)]
pub struct ToggleMap(pub HashMap<String, bool>);

#[derive(Resource, Clone)]
pub enum TargetType {
    Nothing,
    NewLnAddress,
    NewColor,
    NewMessage,
}

#[derive(Resource, Clone)]
pub struct KeyboardTarget(pub TargetType);

// #[derive(Resource, Clone)]
// pub struct AmountSelected(pub u32);

#[derive(Resource, Clone)]
pub struct CurrentCartBlock {
    pub ln_address: String,
    pub color: String,
    pub message: String,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct InvoiceDataFromServer {
    pub invoice: String,
    pub expires: DateTime<Utc>,
    pub code: String,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct InvoiceCheckFromServer {
    pub status: String,
}
