use crate::statey::InitLoadingBlocksState;

use self::{
    event::{ReadLocalBrowserStorage, WriteBrowserStorage},
    localstorage::{readcheck_local_storage, request_local_storage, write_local_storage},
    resource::{tick_browser_receive_timer, BrowserPollingTimer},
};
use bevy::prelude::*;
use localstorage::{readcheck_indexeddb_storage, request_indexeddb_storage};

pub mod event;
pub mod localstorage;
pub mod resource;
pub mod state;

pub struct BrowserPlugin;

impl Plugin for BrowserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrowserPollingTimer>()
            .add_event::<ReadLocalBrowserStorage>()
            .add_event::<WriteBrowserStorage>()
            .add_systems(Update, write_local_storage)
            .add_systems(
                OnEnter(InitLoadingBlocksState::LocalBrowserStorage),
                (request_local_storage).run_if(run_once()),
            )
            .add_systems(
                Update,
                (readcheck_local_storage, tick_browser_receive_timer)
                    .run_if(in_state(InitLoadingBlocksState::LocalBrowserStorage)),
            )
            .add_systems(
                OnEnter(InitLoadingBlocksState::IndexedDB),
                (request_indexeddb_storage).run_if(run_once()),
            )
            .add_systems(
                Update,
                (readcheck_indexeddb_storage, tick_browser_receive_timer)
                    .run_if(in_state(InitLoadingBlocksState::IndexedDB)),
            );
    }
}
//(init_block_loading_text),
