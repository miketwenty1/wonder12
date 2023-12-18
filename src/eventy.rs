use bevy::prelude::*;

use crate::structy::{EdgeType, TileTextType};

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
pub struct UpdateTileTextureEvent;

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
