use bevy::prelude::*;

use crate::statey::InitLoadingBlocksState;

use self::{
    event::{ReadLocalBrowserStorage, WriteLocalBrowserStorage},
    localstorage::{readcheck_local_storage, request_local_storage, write_local_storage},
    resource::{tick_browser_receive_timer, BrowserPollingTimer},
};

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
            //.add_systems(Startup, (setup_comm, ))
            .add_systems(
                Update,
                (
                    write_local_storage,
                    request_local_storage,
                    readcheck_local_storage,
                    tick_browser_receive_timer,
                ),
            );
    }
}
