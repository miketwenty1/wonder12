use bevy::prelude::*;

use crate::CommsState;

use self::{
    api_timer::{tick_api_receive_timer, ApiPollingTimer},
    load_server_data::api_receive_server_tiles,
};

mod api_timer;
pub mod load_server_data;
mod server_structs;
mod setup;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            //.add_systems(Startup, (setup_comm, ))
            .add_systems(
                Update,
                (tick_api_receive_timer, api_receive_server_tiles).run_if(in_state(CommsState::On)),
            );
    }
}
