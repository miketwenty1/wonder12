use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;
use serde::Serialize;

use crate::comms::server_structs::UserGameBlock;
use crate::comms::structy::TrimTile;
use crate::comms::structy::TrimTileLocalBrowserStorage;
use crate::structy::EdgeData;
use crate::structy::TileResource;
use crate::utils::convert_color_to_hexstring;
use chrono::{DateTime, Utc};

#[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldOwnedTileMap {
    pub map: HashMap<u32, TileData>,
}

impl WorldOwnedTileMap {
    pub fn trim_for_browser_storage(&self) -> TrimTileLocalBrowserStorage {
        let trimmed_map: HashMap<u32, TrimTile> = self
            .map
            .iter()
            .map(|(&key, tile_data)| {
                (
                    key,
                    TrimTile {
                        c: convert_color_to_hexstring(tile_data.color), // Assuming a to_hex() method exists for Color
                        v: tile_data.value,
                        h: tile_data.hash.clone(),
                        l: tile_data.ln_address.clone(),
                        m: tile_data.message.clone(),
                        u: tile_data.username.clone(),
                        d: tile_data.event_date,
                    },
                )
            })
            .collect();

        TrimTileLocalBrowserStorage { map: trimmed_map }
    }

    pub fn to_tiledata_vec(&self) -> Vec<TileData>
    where
        TileData: Clone,
    {
        self.map.values().cloned().collect()
    }
}

#[derive(Resource, Clone, Debug)]
pub struct UserPurchasedBlockMessage {
    pub username: String,
    pub value: u32,
    pub message: String,
}

#[derive(Resource, Clone, Debug)]
pub struct TileCartData {
    pub event_date: Option<DateTime<Utc>>,
    pub ln_address: String,
    pub username: String,
    pub color: Option<Color>,
    pub messages: Option<Vec<UserPurchasedBlockMessage>>,
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

#[derive(Resource, Clone, Debug)]
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
    GoTo,
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
    pub nwc: Option<bool>,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct InvoiceCheckFromServer {
    pub status: String,
}

#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
pub struct UpdateGameTimetamp {
    pub ts: DateTime<Utc>,
}

#[derive(Resource, Clone, Debug, Default, Serialize, Deserialize)]
pub struct CheckpointTimetamp {
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
    pub node_color_lighter: Color,
    pub lite_button_color: Color,
    pub button_color: Color,
    pub accent_color: Color,
    pub light_color: Color,
    pub text_color: Color,
    pub red_color: Color,
    pub yellow_color: Color,
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
pub struct UserInventoryBlocks {
    pub ownedblocks: HashMap<u32, UserGameBlock>,
}

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct ToggleVisible(pub bool);

// #[derive(Resource, Clone, Debug, Default, Deserialize)]
// pub struct UserInventoryBlocksFromServer {
//     pub ownedblocks: Vec<UserGameBlock>,
// }
