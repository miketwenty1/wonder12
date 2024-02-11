use self::{
    event::{ReadLocalBrowserStorage, WriteLocalBrowserStorage},
    localstorage::{readcheck_local_storage, request_local_storage, write_local_storage},
    resource::{tick_browser_receive_timer, BrowserPollingTimer},
    state::BrowserStorageState,
};
use bevy::prelude::*;

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
                    .run_if(in_state(BrowserStorageState::On)),
            );
    }
}
//(init_block_loading_text),
