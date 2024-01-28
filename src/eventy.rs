use bevy::prelude::*;

use crate::{
    resourcey::TileData,
    structy::{EdgeType, RequestTileType, TileTextType},
};

#[derive(Event)]
pub struct SpriteSpawnEvent;

#[derive(Event, Debug)]
pub struct EdgeEvent {
    pub edge_type: EdgeType,
    pub x: i32,
    pub y: i32,
}

#[derive(Event, Debug)]
pub struct SelectTileEvent(pub i32, pub i32);

#[derive(Event, Debug)]
pub struct UpdateTileTextureEvent(pub Vec<TileData>);

#[derive(Event, Debug)]
pub struct ToggleBuildings;

#[derive(Event, Debug)]
pub struct ToggleColors;

#[derive(Event, Debug)]
pub struct ToggleText(pub TileTextType);

#[derive(Event, Debug)]
pub struct UpdateUiAmount;

#[derive(Event, Debug)]
pub struct BuyBlockRequest;

#[derive(Event, Debug)]
pub struct RequestTileUpdates(pub RequestTileType);

#[derive(Event, Debug)]
pub struct ClearSelectionEvent;

#[derive(Event, Debug)]
pub struct ClearLastSelectedTile;

#[derive(Event, Debug)]
pub struct KeyboardSpawnEvent;

// #[derive(Debug, Clone)]
// pub struct InvoiceString {
//     pub invoice: String,
// }
#[derive(Event, Debug, Clone)]
pub struct HideBackupCopyBtn;

#[derive(Event, Debug)]
pub struct ShowBackupCopyBtn;

#[derive(Event, Debug)]
pub struct RequestInventoryEvent;

#[derive(Event, Debug)]
pub struct UpdateTilesAfterPurchase;

#[derive(Event, Debug)]
pub struct DespawnInventoryHeights(pub Vec<u32>);
