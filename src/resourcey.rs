use bevy::utils::HashSet;
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
    pub value: u32,
    pub cost: u32,
    pub height: u32,
    pub land_index: usize,
    pub event_date: DateTime<Utc>,
    pub resource: TileResource,
    pub block_hash: String,
    pub block_time: i64,
    pub block_bits: i64,
    pub block_n_tx: i32,
    pub block_size: i32,
    pub block_fee: i64,
    pub block_weight: i64,
    pub block_ver: i32,
}

impl TileData {
    pub fn set_index(&mut self, new_index: usize) {
        self.land_index = new_index;
    }
    pub fn compare_update_fields(&self, other: &TileData) -> bool {
        self.color == other.color
            && self.username == other.username
            && self.ln_address == other.ln_address
            && self.cost == other.cost
            && self.value == other.value
            && self.height == other.height
            && self.land_index == other.land_index
            && self.resource == other.resource
            && self.message == other.message
    }
}

// #[derive(Resource, Clone, PartialEq, Debug, Serialize, Deserialize)]
// pub struct TileDataAggLod {
//     pub color: Color,
//     pub land_index: usize,
//     pub resource: TileResource,
//     pub block_hash: String,
//     pub block_time: i64,
//     pub block_bits: i64,
//     pub block_n_tx: i32,
//     pub block_size: i32,
//     pub block_fee: i64,
//     pub block_weight: i64,
//     pub block_ver: i32,
// }

// impl TileDataAggLod {
//     // Function to merge four TileDataAggLod instances into one, taking the maximum values
//     pub fn from_four(
//         lod1: &TileDataAggLod,
//         lod2: &TileDataAggLod,
//         lod3: &TileDataAggLod,
//         lod4: &TileDataAggLod,
//     ) -> TileDataAggLod {
//         TileDataAggLod {
//             color: max_color(
//                 lod1.color,
//                 max_color(lod2.color, max_color(lod3.color, lod4.color)),
//             ),
//             land_index: *[
//                 lod1.land_index,
//                 lod2.land_index,
//                 lod3.land_index,
//                 lod4.land_index,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             resource: max_resource(
//                 &lod1.resource,
//                 &lod2.resource,
//                 &lod3.resource,
//                 &lod4.resource,
//             ),
//             block_hash: max_block_hash(
//                 &lod1.block_hash,
//                 &lod2.block_hash,
//                 &lod3.block_hash,
//                 &lod4.block_hash,
//             ),
//             block_time: *[
//                 lod1.block_time,
//                 lod2.block_time,
//                 lod3.block_time,
//                 lod4.block_time,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             block_bits: *[
//                 lod1.block_bits,
//                 lod2.block_bits,
//                 lod3.block_bits,
//                 lod4.block_bits,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             block_n_tx: *[
//                 lod1.block_n_tx,
//                 lod2.block_n_tx,
//                 lod3.block_n_tx,
//                 lod4.block_n_tx,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             block_size: *[
//                 lod1.block_size,
//                 lod2.block_size,
//                 lod3.block_size,
//                 lod4.block_size,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             block_fee: *[
//                 lod1.block_fee,
//                 lod2.block_fee,
//                 lod3.block_fee,
//                 lod4.block_fee,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             block_weight: *[
//                 lod1.block_weight,
//                 lod2.block_weight,
//                 lod3.block_weight,
//                 lod4.block_weight,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//             block_ver: *[
//                 lod1.block_ver,
//                 lod2.block_ver,
//                 lod3.block_ver,
//                 lod4.block_ver,
//             ]
//             .iter()
//             .max()
//             .unwrap(),
//         }
//     }

//     // Function to convert TileData to TileDataAggLod
//     pub fn from_tile_data(tile: &TileData) -> TileDataAggLod {
//         TileDataAggLod {
//             color: tile.color,
//             land_index: tile.land_index,
//             resource: tile.resource.clone(),
//             block_hash: tile.block_hash.clone(),
//             block_time: tile.block_time,
//             block_bits: tile.block_bits,
//             block_n_tx: tile.block_n_tx,
//             block_size: tile.block_size,
//             block_fee: tile.block_fee,
//             block_weight: tile.block_weight,
//             block_ver: tile.block_ver,
//         }
//     }
// }

// // Function to get the max color based on perceived brightness
// fn max_color(color1: Color, color2: Color) -> Color {
//     let color1_srgba = color1.to_srgba();
//     let color2_srgba = color2.to_srgba();

//     let brightness1 =
//         color1_srgba.red * 0.299 + color1_srgba.green * 0.587 + color1_srgba.blue * 0.114;
//     let brightness2 =
//         color2_srgba.red * 0.299 + color2_srgba.green * 0.587 + color2_srgba.blue * 0.114;

//     if brightness1 > brightness2 {
//         color1
//     } else {
//         color2
//     }
// }

// // Placeholder function for resource comparison
// fn max_resource(
//     res1: &TileResource,
//     res2: &TileResource,
//     res3: &TileResource,
//     res4: &TileResource,
// ) -> TileResource {
//     // This function should define how you compare and choose the max resource
//     // For now, we'll just return the first one as a placeholder
//     res1.clone()
// }

// // Placeholder function for block_hash comparison
// fn max_block_hash(hash1: &String, hash2: &String, hash3: &String, hash4: &String) -> String {
//     // This function should define how you compare and choose the max block hash
//     // For now, we'll just return the first one as a placeholder
//     hash1.clone()
// }

impl Default for TileData {
    fn default() -> Self {
        TileData {
            ln_address: String::new(),
            username: String::new(),
            color: Srgba::hex("666666").unwrap().into(),
            message: String::new(),
            value: 0,
            cost: 0,
            height: 0,
            land_index: 0,
            event_date: Utc::now(),
            resource: TileResource::default(),
            block_hash: String::new(),
            block_time: 0,
            block_bits: 0,
            block_n_tx: 0,
            block_size: 0,
            block_fee: 0,
            block_weight: 0,
            block_ver: 0,
        }
    }
}

#[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldOwnedTileMap {
    pub map: HashMap<u32, TileData>,
}

// #[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
// pub struct WorldOwnedTileMapLod1 {
//     pub map: HashMap<(i32, i32), TileDataAggLod>,
// }

// impl WorldOwnedTileMapLod1 {
//     pub fn to_lod2(&self) -> WorldOwnedTileMapLod2 {
//         let mut lod2_map: HashMap<(i32, i32), TileDataAggLod> = HashMap::new();

//         // Iterate over the lod1_map and group tiles into 2x2 chunks
//         for (&(x, y), tile_data) in &self.map {
//             // Determine the top-left corner of the 2x2 block for LOD2
//             let lod2_x = (x / 2) * 2;
//             let lod2_y = (y / 2) * 2;

//             // Aggregate the tile data into the LOD2 map
//             let entry = lod2_map
//                 .entry((lod2_x, lod2_y))
//                 .or_insert_with(|| tile_data.clone()); // Initialize with the first tile's data

//             // Update the LOD2 entry with the maximum values from the 2x2 block
//             entry.color = max_color(entry.color, tile_data.color);
//             entry.block_time = entry.block_time.max(tile_data.block_time);
//             entry.block_bits = entry.block_bits.max(tile_data.block_bits);
//             entry.block_n_tx = entry.block_n_tx.max(tile_data.block_n_tx);
//             entry.block_size = entry.block_size.max(tile_data.block_size);
//             entry.block_fee = entry.block_fee.max(tile_data.block_fee);
//             entry.block_weight = entry.block_weight.max(tile_data.block_weight);
//             entry.block_ver = entry.block_ver.max(tile_data.block_ver);
//         }

//         WorldOwnedTileMapLod2 { map: lod2_map }
//     }
// }

// #[derive(Resource, Clone, PartialEq, Serialize, Deserialize)]
// pub struct WorldOwnedTileMapLod2 {
//     pub map: HashMap<(i32, i32), TileDataAggLod>,
// }

impl WorldOwnedTileMap {
    pub fn trim_for_browser_storage(&self) -> TrimTileLocalBrowserStorage {
        let trimmed_map: HashMap<u32, TrimTile> = self
            .map
            .iter()
            .filter(|(_, tile_data)| tile_data.value != 0)
            .map(|(&key, tile_data)| {
                (
                    key,
                    TrimTile {
                        c: convert_color_to_hexstring(tile_data.color.to_srgba()),
                        v: tile_data.value,
                        l: tile_data.ln_address.clone(),
                        m: tile_data.message.clone(),
                        u: tile_data.username.clone(),
                        d: tile_data.event_date,
                        h: tile_data.block_hash.clone(),
                        bt: tile_data.block_time,
                        bb: tile_data.block_bits,
                        bn: tile_data.block_n_tx,
                        bs: tile_data.block_size,
                        bf: tile_data.block_fee,
                        bw: tile_data.block_weight,
                        bv: tile_data.block_ver,
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
    // pub fn to_xy_map(&self) -> HashMap<(i32, i32), TileData> {
    //     let mut xy_map: HashMap<(i32, i32), TileData> = HashMap::new();

    //     for (&height, tile_data) in &self.map {
    //         // Convert height to (x, y) using the provided function
    //         let (x, y) = ulam::calc_coord::calc_xy(height);
    //         // Insert the (x, y) coordinate as the key in the new map
    //         xy_map.insert((x, y), tile_data.clone());
    //     }

    //     xy_map
    // }

    // this should contain a 1/4 amount of entries as WorldOwnedTileMap
    // storing 2x2 aggregates
    // pub fn to_lod1(&self) -> WorldOwnedTileMapLod1 {
    //     let mut lod1_map: HashMap<(i32, i32), TileDataAggLod> = HashMap::new();

    //     // Convert the map to (x, y) keys
    //     let xy_map = self.to_xy_map();

    //     // Iterate over the xy_map and group tiles into 2x2 chunks
    //     for (&(x, y), tile_data) in &xy_map {
    //         // Determine the top-left corner of the 2x2 block
    //         let lod1_x = (x / 2) * 2;
    //         let lod1_y = (y / 2) * 2;

    //         // Aggregate the tile data into the LOD1 map
    //         let entry = lod1_map
    //             .entry((lod1_x, lod1_y))
    //             .or_insert_with(|| TileDataAggLod::from_tile_data(tile_data));

    //         // Update the LOD1 entry with the maximum values from the 2x2 block
    //         entry.color = max_color(entry.color, tile_data.color);
    //         entry.block_time = entry.block_time.max(tile_data.block_time);
    //         entry.block_bits = entry.block_bits.max(tile_data.block_bits);
    //         entry.block_n_tx = entry.block_n_tx.max(tile_data.block_n_tx);
    //         entry.block_size = entry.block_size.max(tile_data.block_size);
    //         entry.block_fee = entry.block_fee.max(tile_data.block_fee);
    //         entry.block_weight = entry.block_weight.max(tile_data.block_weight);
    //         entry.block_ver = entry.block_ver.max(tile_data.block_ver);
    //     }

    //     WorldOwnedTileMapLod1 { map: lod1_map }
    // }
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
    pub set: HashSet<u32>,
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

// #[derive(Resource, Clone)]
// pub struct SpriteSheetBg {
//     pub layout: Handle<TextureAtlasLayout>,
//     pub texture: Handle<Image>,
// }

#[derive(Resource, Clone)]
pub struct SpriteSheetSelect {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource, Clone)]
pub struct SpriteSheetLand {
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
    pub error_msg: Option<String>,
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
pub struct LocalBrowserStorageCount(pub u32);

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

// #[derive(Resource, Clone, Debug, Default, Deserialize)]
// pub struct MultiTouchInfo {
//     //pub status: bool,
//     pub distance: f32,
// }

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct ConfigAllCartBlocks(pub bool);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct IsIphone(pub bool);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct BlockExplorerCount(pub u32);

#[derive(Resource, Clone, Debug, Default, Deserialize)]
pub struct Nwc(pub bool);

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

#[derive(Resource, Clone, Debug)]
pub struct UiInteracting(pub bool);

#[derive(Resource, Clone, Debug)]
pub struct MapTileMode(pub ColorMapToggle);

#[derive(Resource, Clone, Debug, PartialEq)]
pub enum ColorMapToggle {
    GameColor,
    LandTile,
    Fee,
    BlockTime,
    TxCount,
    Byte,
    Weight,
    TargetDifficulty,
    LeadingZeros,
    ExcessWork,
    Version,
}

#[derive(Resource, Clone, PartialEq, Debug)]
pub enum ZoomSpawnEvent {
    NoText,
    NoBuildings,
    YesText,
    YesBuildings,
}
