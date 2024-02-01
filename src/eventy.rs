use bevy::prelude::*;

use crate::resourcey::TileData;

#[derive(Event)]
pub struct SpriteSpawnEvent;

#[derive(Event, Debug)]
pub struct UpdateTileTextureEvent(pub Vec<TileData>);
