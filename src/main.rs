use crate::consty::{CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT};
use crate::eventy::{SpriteSpawnEvent, UpdateTileTextureEvent};
use crate::explore_scene::ExplorePlugin;

use crate::resourcey::{ChunkManager, Edge, WorldOwnedTileMap};
use crate::statey::ExploreState;
use crate::structy::EdgeData;
use bevy::asset::AssetMetaCheck;

// use bevy::window::WindowResolution;
use bevy::{prelude::*, utils::HashMap};

use spritesheetfns::setup_spritesheets;
use wasm_bindgen::prelude::wasm_bindgen;

mod explore_scene;

mod componenty;
mod consty;
mod eventy;
mod resourcey;
mod spritesheetfns;
mod statey;
mod structy;

pub fn main() {
    game12();
}
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn game12() {
    let start_edge = Edge {
        top: EdgeData {
            pixel: CHUNK_PIXEL_SIZE / 2.0,
            tile: CHUNK_TILE_SPAN_COUNT,
        },
        bottom: EdgeData {
            pixel: -CHUNK_PIXEL_SIZE / 2.0,
            tile: -CHUNK_TILE_SPAN_COUNT,
        },
        left: EdgeData {
            pixel: -CHUNK_PIXEL_SIZE / 2.0,
            tile: -CHUNK_TILE_SPAN_COUNT,
        },
        right: EdgeData {
            pixel: CHUNK_PIXEL_SIZE / 2.0,
            tile: CHUNK_TILE_SPAN_COUNT,
        },
    };

    let window = Window {
        title: "SatoshiSettlers".to_string(),
        ..default()
    };

    App::new()
        .insert_resource(start_edge)
        .insert_resource(ChunkManager {
            map: HashMap::new(),
        })
        .insert_resource(WorldOwnedTileMap {
            map: HashMap::new(),
        })
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<ExploreState>()
        .add_plugins(ExplorePlugin)
        .add_event::<SpriteSpawnEvent>()
        .add_event::<UpdateTileTextureEvent>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, (setup2, setup_spritesheets).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn setup2(mut ui_state: ResMut<NextState<ExploreState>>) {
    ui_state.set(ExploreState::On);
}
