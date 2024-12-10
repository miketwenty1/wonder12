use crate::eventy::{
    BuyBlockRequest, ClearLastSelectedTile, ClearSelectionEvent, EdgeEvent, RequestTileUpdates,
    SelectTileEvent, SpriteSpawnEvent, ToggleBuildings, ToggleColors, ToggleText,
    UpdateTileTextureEvent, UpdateUiAmount,
};

use crate::keyboard::resources::KeyboardData;
use crate::keyboard::KeyboardState;
use crate::resourcey::{
    ChunkManager, CurrentCartBlock, InvoiceCheckFromServer, InvoiceDataFromServer,
    LastSelectedTile, MaxBlockHeight, ServerURL, TargetType, TileCart, TileCartVec,
    UpdateGameTimetamp, User, WorldOwnedTileMap,
};
use crate::statey::{CommsApiState, DisplayBuyUiState, ExploreSceneState, ExploreSelectState};
use bevy::asset::AssetMetaCheck;

use bevy::color::palettes::css::DARK_GRAY;
use bevy::utils::HashSet;
// use bevy::window::WindowResolution;
use bevy::{prelude::*, utils::HashMap};
use browser::event::ReadIndexedDBStorage;
use chrono::{Duration, Utc};
use eventy::{
    BlockDetailMessage, ClearManualSelectionEvent, DespawnInventoryHeights, HideBackupCopyBtn,
    KeyboardSpawnEvent, MessageReceivedFromServer, NumberKeyboardSpawnEvent, RequestInventoryEvent,
    ShowBackupCopyBtn, TravelHeight, UpdateTilesAfterPurchase,
};
use explore_scene::core_ui::inventory::state::InventoryUiState;
use explore_scene::core_ui::paint_palette::state::{PaintPaletteUiState, ToolPaletteUiState};
use explore_scene::overlay_ui::go_to::state::GoToUiState;
use init_scene::InitPlugin;
use resourcey::{
    BlockExplorerCount, CheckpointTimetamp, ColorMapToggle, ConfigAllCartBlocks, InitGameMap,
    IsIphone, LocalBrowserStorageCount, MapTileMode, Nwc, ToggleVisible, UiInteracting,
    UserInventoryBlocks, WinSize,
};
use statey::{
    CommsApiBlockLoadState, CommsApiInventoryState, InitLoadingBlocksState, InitSceneState,
    ToastState,
};
use wasm_bindgen::prelude::wasm_bindgen;

mod browser;
mod building_config;
mod comms;
mod componenty;
mod consty;
mod eventy;
mod explore_scene;
mod init_scene;
mod keyboard;
mod resourcey;
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
    nwc: bool,
    block_explorer_count: u32,
) {
    let window = Window {
        title: "SatoshiSettlers".to_string(),
        ..default()
    };

    App::new()
        .insert_resource(MapTileMode(ColorMapToggle::GameColor))
        .insert_resource(UiInteracting(false))
        .insert_resource(MaxBlockHeight(max_height))
        .insert_resource(ChunkManager {
            set: HashSet::new(),
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
            color: DARK_GRAY.into(), // this is just a place holder shouldn't be used.
            message: "".to_string(),
        })
        .insert_resource(LastSelectedTile(1_000_000, 1_000_000))
        .insert_resource(ServerURL(server_url))
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
        .insert_resource(LocalBrowserStorageCount(block_init_count))
        .insert_resource(UpdateGameTimetamp {
            ts: Utc::now() - Duration::minutes(5),
        })
        .insert_resource(CheckpointTimetamp { ts: Utc::now() })
        .insert_resource(InitGameMap { height: 0 })
        .init_resource::<InvoiceDataFromServer>()
        .init_resource::<InvoiceCheckFromServer>()
        //.init_resource::<DrawState>()
        .insert_resource(WinSize {
            width: viewport_width as f32,
            height: viewport_height as f32,
        })
        .insert_resource(ConfigAllCartBlocks(false))
        .insert_resource(IsIphone(is_iphone))
        .insert_resource(BlockExplorerCount(block_explorer_count))
        .insert_resource(Nwc(nwc))
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
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .init_state::<ExploreSceneState>()
        .init_state::<InitSceneState>()
        .init_state::<ExploreSelectState>()
        .init_state::<CommsApiState>()
        .init_state::<CommsApiBlockLoadState>()
        .init_state::<CommsApiInventoryState>()
        .init_state::<DisplayBuyUiState>()
        .init_state::<KeyboardState>()
        .init_state::<InitLoadingBlocksState>()
        .init_state::<ToastState>()
        .init_state::<InventoryUiState>()
        // .init_state::<BrowserLocalStorageState>()
        // .init_state::<BrowserIndexedDBStorageState>()
        .init_state::<GoToUiState>()
        .init_state::<PaintPaletteUiState>()
        .init_state::<ToolPaletteUiState>()
        .add_plugins(InitPlugin)
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
        .add_event::<ClearManualSelectionEvent>()
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
        .add_event::<ReadIndexedDBStorage>()
        // .add_systems(Startup, load_textures)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    //mut browser_check: EventWriter<ReadLocalBrowserStorage>,
    //mut explorer_check: EventWriter<ReadIndexedDBStorage>,
) {
    info!(
        "this is the init value for game ts: {}",
        Utc::now() - Duration::minutes(5)
    );
    commands.spawn(Camera2dBundle::default());
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
