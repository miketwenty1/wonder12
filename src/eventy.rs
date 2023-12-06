use bevy::prelude::*;

use crate::structy::EdgeType;

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
