use crate::comms::CommsPlugin;
use crate::consty::{CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT};
use crate::eventy::{
    BuyBlockRequest, EdgeEvent, RequestTileUpdates, SelectTileEvent, SpriteSpawnEvent,
    ToggleBuildings, ToggleColors, ToggleText, UpdateTileTextureEvent, UpdateUiAmount,
};
use crate::explore_scene::ExplorePlugin;
use crate::keyboard::resources::KeyboardData;
use crate::keyboard::{KeyboardPlugin, KeyboardState};
use crate::overlay_ui::OverlayUiPlugin;
use crate::resourcey::{
    ChunkManager, ColorPalette, CurrentCartBlock, Edge, InvoiceCheckFromServer,
    InvoiceDataFromServer, KeyboardTarget, LastSelectedTile, MaxBlockHeight, ServerURL,
    SpriteIndexBuilding, TargetType, TileCart, TileCartVec, TileDataChannel, TileMap, ToggleMap,
    UpdateGameTimetamp, User,
};
use crate::statey::{CommsApiState, DisplayBuyUiState, ExploreState};
use crate::structy::EdgeData;
use bevy::asset::AssetMetaCheck;
use bevy::{prelude::*, utils::HashMap};
use chrono::{Duration, Utc};
use resourcey::{CheckInvoiceChannel, RequestInvoiceChannel};
use wasm_bindgen::prelude::wasm_bindgen;

mod building_config;
mod comms;
mod explore_scene;
mod keyboard;
mod overlay_ui;

mod componenty;
mod consty;
mod eventy;
mod resourcey;
mod statey;
mod structy;

mod utils;

pub fn main() {
    //game("localusertesting".to_string(), "localhost:8081".to_string());
}
#[wasm_bindgen]
pub fn game12(username: String, server_url: String, ln_address: String) {
    let max_block_height: u32 = 840_000;
    let mut toggle_map = HashMap::new();
    toggle_map.insert("hidebuildings".to_string(), true);
    toggle_map.insert("showbuildings".to_string(), false);
    toggle_map.insert("showcolors".to_string(), true);
    toggle_map.insert("hidecolors".to_string(), false);
    toggle_map.insert("showvalues".to_string(), true);
    toggle_map.insert("showheights".to_string(), false);
    toggle_map.insert("showtext".to_string(), false);
    toggle_map.insert("hidetext".to_string(), true);

    let mut numbers_map = HashMap::new();

    numbers_map.insert(0, 0);
    numbers_map.insert(128, 1);
    numbers_map.insert(256, 2);
    numbers_map.insert(512, 3);
    numbers_map.insert(1024, 3);
    numbers_map.insert(2048, 3);
    numbers_map.insert(4096, 3);
    numbers_map.insert(8192, 3);
    numbers_map.insert(16384, 3);
    numbers_map.insert(32768, 3);
    numbers_map.insert(65536, 3);
    numbers_map.insert(131072, 3);
    numbers_map.insert(262144, 3);
    numbers_map.insert(524288, 3);
    numbers_map.insert(1048576, 3);

    let color_palette = ColorPalette {
        node_color: Color::hex("222831").unwrap(),
        button_color: Color::hex("393E46").unwrap(),
        accent_color: Color::hex("00ADB5").unwrap(),
        light_color: Color::hex("EEEEEE").unwrap(),
        text_color: Color::hex("FAFAFA").unwrap(),
    };
    info!(
        "user: {}\nserver: {}, lnaddress: {}",
        username, server_url, ln_address
    );
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

    App::new()
        .insert_resource(start_edge)
        .insert_resource(color_palette)
        .insert_resource(MaxBlockHeight(max_block_height))
        .insert_resource(ChunkManager {
            map: HashMap::new(),
        })
        .insert_resource(TileMap {
            map: HashMap::new(),
        })
        .insert_resource(TileCart {
            map: HashMap::new(),
        })
        .insert_resource(TileCartVec {
            vec: Vec::new(),
            index: 0,
        })
        .insert_resource(CurrentCartBlock {
            ln_address: "".to_string(),
            color: "".to_string(),
            message: "".to_string(),
        })
        .insert_resource(LastSelectedTile(1_000_000, 1_000_000))
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ServerURL(server_url))
        .insert_resource(SpriteIndexBuilding(numbers_map))
        .insert_resource(ToggleMap(toggle_map))
        .insert_resource(KeyboardData("".to_string()))
        .insert_resource(KeyboardTarget(TargetType::Nothing))
        .insert_resource(User {
            name: username,
            ln_address,
        })
        .insert_resource(UpdateGameTimetamp {
            ts: Utc::now() - Duration::days(10 * 365),
        })
        .init_resource::<InvoiceDataFromServer>()
        .init_resource::<InvoiceCheckFromServer>()
        //.add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        //resolution: [width as f32, height as f32].into(),
                        fit_canvas_to_parent: true,
                        title: "SatoshiSettlers".to_string(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //default_nearest()),
        )
        .add_state::<ExploreState>()
        .add_state::<CommsApiState>()
        .add_state::<DisplayBuyUiState>()
        .add_state::<KeyboardState>()
        .add_plugins(CommsPlugin)
        .add_plugins(OverlayUiPlugin)
        .add_plugins(ExplorePlugin)
        .add_plugins(KeyboardPlugin)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        //.insert_resource(WinitSettings::desktop_app())
        .add_event::<EdgeEvent>()
        .add_event::<SpriteSpawnEvent>()
        .add_event::<UpdateTileTextureEvent>()
        .add_event::<SelectTileEvent>()
        .add_event::<ToggleBuildings>()
        .add_event::<ToggleColors>()
        .add_event::<ToggleText>()
        .add_event::<UpdateUiAmount>()
        .add_event::<BuyBlockRequest>()
        .add_event::<RequestTileUpdates>()
        .add_systems(Startup, setup) //setupcoords
        //.add_systems(PostStartup, api_get_server_tiles)
        .run();
}

fn setup(
    mut commands: Commands,
    mut ui_state: ResMut<NextState<ExploreState>>,
    mut request_tiles_event: EventWriter<RequestTileUpdates>,
) {
    commands.spawn(Camera2dBundle::default());
    let (tx_tiledata, rx_tiledata) = async_channel::bounded(1);
    commands.insert_resource(TileDataChannel {
        tx: tx_tiledata,
        rx: rx_tiledata,
    });
    let (tx_tiledata, rx_tiledata) = async_channel::bounded(1);
    commands.insert_resource(RequestInvoiceChannel {
        tx: tx_tiledata,
        rx: rx_tiledata,
    });
    let (tx_tiledata, rx_tiledata) = async_channel::bounded(1);
    commands.insert_resource(CheckInvoiceChannel {
        tx: tx_tiledata,
        rx: rx_tiledata,
    });
    request_tiles_event.send(RequestTileUpdates);
    ui_state.set(ExploreState::On);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
