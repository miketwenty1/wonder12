use bevy::prelude::*;

use crate::{
    comms::async_resource_comm_channels::{
        BlockMessagesStorageChannel, BrowserCheckpointLocalStorageChannel,
        BrowserIndexedDBStorageChannel, BrowserMapLocalStorageChannel, CheckInvoiceChannel,
        RequestInvoiceChannel, TileDataChannel, UserBlockInventoryChannel,
    },
    eventy::RequestInventoryEvent,
    statey::{ExploreSceneState, ExploreSelectState, InitLoadingBlocksState, InitSceneState},
};

pub fn init_js_comms_channels(
    mut commands: Commands,
    mut ui_state: ResMut<NextState<ExploreSceneState>>,
    mut load_browser_state: ResMut<NextState<InitLoadingBlocksState>>,
    mut ui_select_state: ResMut<NextState<ExploreSelectState>>,
    mut request_inventory_event: EventWriter<RequestInventoryEvent>,
    mut init_state: ResMut<NextState<InitSceneState>>,
) {
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
    commands.insert_resource(BrowserIndexedDBStorageChannel { tx, rx });
    let (tx, rx) = async_channel::bounded(1);
    commands.insert_resource(BrowserCheckpointLocalStorageChannel { tx, rx });

    let (tx, rx) = async_channel::bounded(10);
    commands.insert_resource(BlockMessagesStorageChannel { tx, rx });

    // request_tiles_event.send(RequestTileUpdates(RequestTileType::Height));
    //browser_check.send(ReadLocalBrowserStorage);
    //explorer_check.send(ReadIndexedDBStorage);
    info!("trigger the rest");
    load_browser_state.set(InitLoadingBlocksState::LocalBrowserStorage);
    request_inventory_event.send(RequestInventoryEvent);
    ui_state.set(ExploreSceneState::On);
    ui_select_state.set(ExploreSelectState::On);

    init_state.set(InitSceneState::Done);
}
