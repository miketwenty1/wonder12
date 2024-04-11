use crate::async_resource_comm_channels::{
    BlockMessagesStorageChannel, BrowserCheckpointLocalStorageChannel,
    BrowserMapLocalStorageChannel, CheckInvoiceChannel, RequestInvoiceChannel, TileDataChannel,
    UserBlockInventoryChannel,
};
use crate::comms::CommsPlugin;
use crate::consty::{CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT};
use crate::eventy::{
    BuyBlockRequest, ClearLastSelectedTile, ClearSelectionEvent, EdgeEvent, RequestTileUpdates,
    SelectTileEvent, SpriteSpawnEvent, ToggleBuildings, ToggleColors, ToggleText,
    UpdateTileTextureEvent, UpdateUiAmount,
};
use crate::explore_scene::ExplorePlugin;
use crate::keyboard::resources::KeyboardData;
use crate::keyboard::{KeyboardPlugin, KeyboardState};
use crate::overlay_ui::OverlayUiPlugin;
use crate::resourcey::{
    ChunkManager, ColorPalette, CurrentCartBlock, Edge, InvoiceCheckFromServer,
    InvoiceDataFromServer, LastSelectedTile, MaxBlockHeight, ServerURL, SpriteIndexBuilding,
    TargetType, TileCart, TileCartVec, ToggleMap, UpdateGameTimetamp, User, WorldOwnedTileMap,
};
use crate::statey::{CommsApiState, DisplayBuyUiState, ExploreSelectState, ExploreState};
use crate::structy::EdgeData;
use bevy::asset::AssetMetaCheck;

// use bevy::window::WindowResolution;
use bevy::{prelude::*, utils::HashMap};
use browser::event::ReadLocalBrowserStorage;
use browser::state::BrowserStorageState;
use browser::BrowserPlugin;
use chrono::{Duration, Utc};
use eventy::{
    BlockDetailMessage, DespawnInventoryHeights, HideBackupCopyBtn, KeyboardSpawnEvent,
    MessageReceivedFromServer, NumberKeyboardSpawnEvent, RequestInventoryEvent, ShowBackupCopyBtn,
    TravelHeight, UpdateTilesAfterPurchase,
};
use overlay_ui::go_to::state::GoToUiState;
use overlay_ui::inventory::state::InventoryUiState;
use resourcey::{
    CheckpointTimetamp, ConfigAllCartBlocks, InitBlockCount, InitGameMap, IsIphone, MultiTouchInfo,
    ToggleVisible, UserInventoryBlocks, WinSize,
};
use spritesheetfns::setup_spritesheets;
use statey::{CommsApiBlockLoadState, CommsApiInventoryState, InitLoadingBlocksState, ToastState};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

mod building_config;
mod comms;
mod explore_scene;
mod keyboard;
mod overlay_ui;

mod async_resource_comm_channels;
mod browser;
mod componenty;
mod consty;
mod eventy;
mod resourcey;
mod spritesheetfns;
mod statey;
mod structy;
mod utils;

pub fn main() {
    //game("localusertesting".to_string(), "localhost:8081".to_string());
}
#[allow(clippy::too_many_arguments)]
#[wasm_bindgen]
pub fn game12(
    username: String,
    server_url: String,
    ln_address: String,
    block_init_count: u32,
    max_height: u32,
    viewport_width: u32,
    viewport_height: u32,
    _screen_width: u32,
    _screen_height: u32,
    _device_pixel_ratio: f32,
    is_iphone: bool,
) {
    let mut toggle_map = HashMap::new();
    toggle_map.insert("showbuildings".to_string(), false);
    toggle_map.insert("showcolors".to_string(), false);
    toggle_map.insert("showvalues".to_string(), true);
    toggle_map.insert("showheights".to_string(), false);
    toggle_map.insert("showtext".to_string(), false);

    let mut numbers_map = HashMap::new();

    numbers_map.insert(0, 0);
    numbers_map.insert(128, 1);
    numbers_map.insert(256, 2);
    numbers_map.insert(512, 3);
    numbers_map.insert(1024, 4);
    numbers_map.insert(2048, 5);
    numbers_map.insert(4096, 6);
    numbers_map.insert(8192, 7);
    numbers_map.insert(16384, 8);
    numbers_map.insert(32768, 9);
    numbers_map.insert(65536, 10);
    numbers_map.insert(131072, 10);
    numbers_map.insert(262144, 10);
    numbers_map.insert(524288, 10);
    numbers_map.insert(1048576, 10);

    let color_palette = ColorPalette {
        node_color: LegacyColor::hex("222831").unwrap(),
        node_color_lighter: LegacyColor::hex("353d48").unwrap(),
        button_color: LegacyColor::hex("393E46").unwrap(),
        lite_button_color: LegacyColor::hex("6A7382").unwrap(),
        accent_color: LegacyColor::hex("00ADB5").unwrap(),
        light_color: LegacyColor::hex("EEEEEE").unwrap(),
        text_color: LegacyColor::hex("FAFAFA").unwrap(),
        red_color: LegacyColor::hex("B50800").unwrap(),
        yellow_color: LegacyColor::hex("ADB500").unwrap(),
        green_color: LegacyColor::DARK_GREEN,
    };

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
        .insert_resource(color_palette)
        .insert_resource(MaxBlockHeight(max_height))
        .insert_resource(ChunkManager {
            map: HashMap::new(),
        })
        .insert_resource(WorldOwnedTileMap {
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
            ln_address: ln_address.clone(),
            color_text: "".to_string(),
            color: LegacyColor::DARK_GRAY, // this is just a place holder shouldn't be used.
            message: "".to_string(),
        })
        .insert_resource(LastSelectedTile(1_000_000, 1_000_000))
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ServerURL(server_url))
        .insert_resource(SpriteIndexBuilding(numbers_map))
        .insert_resource(ToggleMap(toggle_map))
        .insert_resource(ToggleVisible(false))
        .insert_resource(KeyboardData {
            value: "".to_string(),
            target: TargetType::Nothing,
        })
        //.insert_resource(KeyboardTarget(TargetType::Nothing))
        .insert_resource(User {
            name: username,
            ln_address,
        })
        .insert_resource(InitBlockCount(block_init_count))
        .insert_resource(UpdateGameTimetamp {
            ts: Utc::now() - Duration::minutes(5),
        })
        .insert_resource(CheckpointTimetamp { ts: Utc::now() })
        .insert_resource(InitGameMap { height: 0 })
        .init_resource::<InvoiceDataFromServer>()
        .init_resource::<InvoiceCheckFromServer>()
        .insert_resource(WinSize {
            width: viewport_width as f32,
            height: viewport_height as f32,
        })
        .insert_resource(MultiTouchInfo {
            //status: false,
            distance: 0.0,
        })
        .insert_resource(ConfigAllCartBlocks(false))
        .insert_resource(IsIphone(is_iphone))
        .insert_resource(UserInventoryBlocks {
            ownedblocks: HashMap::new(),
        })
        //.add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()), //default_nearest()),
        )
        .init_state::<ExploreState>()
        .init_state::<ExploreSelectState>()
        .init_state::<CommsApiState>()
        .init_state::<CommsApiBlockLoadState>()
        .init_state::<CommsApiInventoryState>()
        .init_state::<DisplayBuyUiState>()
        .init_state::<KeyboardState>()
        .init_state::<InitLoadingBlocksState>()
        .init_state::<ToastState>()
        .init_state::<InventoryUiState>()
        .init_state::<BrowserStorageState>()
        .init_state::<GoToUiState>()
        .add_plugins((
            CommsPlugin,
            OverlayUiPlugin,
            ExplorePlugin,
            KeyboardPlugin,
            BrowserPlugin,
        ))
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
        .add_event::<ClearSelectionEvent>()
        .add_event::<ClearLastSelectedTile>()
        .add_event::<KeyboardSpawnEvent>()
        .add_event::<NumberKeyboardSpawnEvent>()
        .add_event::<HideBackupCopyBtn>()
        .add_event::<ShowBackupCopyBtn>()
        .add_event::<RequestInventoryEvent>()
        .add_event::<UpdateTilesAfterPurchase>()
        .add_event::<DespawnInventoryHeights>()
        .add_event::<BlockDetailMessage>()
        .add_event::<MessageReceivedFromServer>()
        .add_event::<TravelHeight>()
        // .add_systems(Startup, load_textures)
        .add_systems(Startup, (setup_spritesheets, setup).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut ui_state: ResMut<NextState<ExploreState>>,
    mut ui_select_state: ResMut<NextState<ExploreSelectState>>,
    mut request_inventory_event: EventWriter<RequestInventoryEvent>,
    mut browser_check: EventWriter<ReadLocalBrowserStorage>,
) {
    info!(
        "this is the init value for game ts: {}",
        Utc::now() - Duration::minutes(5)
    );
    fit_canvas_to_parent();
    commands.spawn(Camera2dBundle::default());

    let (tx, rx) = async_channel::bounded(4);
    commands.insert_resource(TileDataChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(RequestInvoiceChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(CheckInvoiceChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(UserBlockInventoryChannel { tx, rx });

    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(BrowserMapLocalStorageChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(BrowserCheckpointLocalStorageChannel { tx, rx });

    let (tx, rx) = async_channel::bounded(10);
    commands.insert_resource(BlockMessagesStorageChannel { tx, rx });

    // request_tiles_event.send(RequestTileUpdates(RequestTileType::Height));
    browser_check.send(ReadLocalBrowserStorage);
    request_inventory_event.send(RequestInventoryEvent);
    ui_state.set(ExploreState::On);
    ui_select_state.set(ExploreSelectState::On);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn fit_canvas_to_parent() {
    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .unchecked_into();
    let style = canvas.style();
    style.set_property("width", "100%").unwrap();
    style.set_property("height", "100%").unwrap();
}
