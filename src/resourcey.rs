use async_channel::{Receiver, Sender};
use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;

use crate::comms::server_structs::UserGameBlock;
use crate::structy::EdgeData;
use crate::structy::TileResource;
use chrono::{DateTime, Utc};

#[derive(Resource, Clone, PartialEq, Debug)]
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

#[derive(Resource, Clone, PartialEq)]
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

// #[derive(Resource, Deref, DerefMut, Clone)]
// pub struct SpriteSheetBuildingRes(pub Handle<TextureAtlas>);

#[derive(Resource, Clone)]
pub struct SpriteSheetBuilding {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct SpriteSheetBg {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct SpriteSheetSelect {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

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
pub struct UserBlockInventory {
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

#[derive(Resource, Clone, PartialEq)]
pub enum TargetType {
    Nothing,
    NewLnAddress,
    NewColor,
    NewMessage,
}

// #[derive(Resource, Clone)]
// pub struct AmountSelected(pub u32);

#[derive(Resource, Clone, Debug)]
pub struct CurrentCartBlock {
    pub ln_address: String,
    pub color_text: String,
    pub color: Color,
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

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct UpdateGameTimetamp {
    pub ts: DateTime<Utc>,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct InitGameMap {
    pub height: u32,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct InitBlockCount(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct ColorPalette {
    pub node_color: Color,
    pub lite_button_color: Color,
    pub button_color: Color,
    pub accent_color: Color,
    pub light_color: Color,
    pub text_color: Color,
    pub red_color: Color,
    pub green_color: Color,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct MaxBlockHeight(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct MultiTouchInfo {
    //pub status: bool,
    pub distance: f32,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct ConfigAllCartBlocks(pub bool);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct IsIphone(pub bool);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct InventoryBlocks {
    pub ownedblocks: Vec<UserGameBlock>,
}
