use async_channel::{Receiver, Sender};
use bevy::{prelude::*, utils::HashMap};

use crate::structy::EdgeData;
use crate::structy::TileResource;

#[derive(Resource, Clone)]
pub struct TileData {
    pub ln_address: String,
    pub owner: String,
    pub color: Color,
    pub message: String,
    pub resource: TileResource,
    pub hash: String,
    pub amount: u32,
    pub height: u32,
}

#[derive(Resource, Clone)]
pub struct TileMap {
    pub map: HashMap<u32, TileData>,
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
pub struct ServerURL(pub String);
