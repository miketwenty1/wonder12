use self::{
    event::{ReadLocalBrowserStorage, WriteLocalBrowserStorage},
    localstorage::{readcheck_local_storage, request_local_storage, write_local_storage},
    resource::{tick_browser_receive_timer, BrowserPollingTimer},
    state::BrowserLocalStorageState,
};
use bevy::prelude::*;
use localstorage::{readcheck_indexeddb_storage, request_indexeddb_storage};
use state::BrowserIndexedDBStorageState;

pub mod event;
pub mod localstorage;
pub mod resource;
pub mod state;

pub struct BrowserPlugin;

impl Plugin for BrowserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BrowserPollingTimer>()
            .add_event::<ReadLocalBrowserStorage>()
            .add_event::<WriteLocalBrowserStorage>()
            .add_systems(Update, write_local_storage)
            .add_systems(
                Update,
                (
                    request_local_storage,
                    readcheck_local_storage,
                    tick_browser_receive_timer,
                )
                    .run_if(in_state(BrowserLocalStorageState::On)),
            )
            .add_systems(
                OnEnter(BrowserIndexedDBStorageState::On),
                (request_indexeddb_storage).run_if(run_once()),
            )
            .add_systems(
                Update,
                (readcheck_indexeddb_storage, tick_browser_receive_timer)
                    .run_if(in_state(BrowserIndexedDBStorageState::On)),
            );
    }
}
//(init_block_loading_text),
