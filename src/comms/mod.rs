use bevy::prelude::*;

use crate::{
    statey::{CommsApiBlockLoadState, CommsApiInventoryState},
    CommsApiState,
};

use self::{
    api_timer::{tick_api_receive_timer, ApiPollingTimer},
    invoice::{
        api_check_invoice, api_receive_invoice, api_receive_invoice_check, api_request_invoice,
        hide_backup_copy_btn, show_backup_copy_btn,
    },
    load_server_data::{api_get_server_tiles, api_receive_server_tiles},
    user_inventory_blocks::{api_get_inventory_blocks, api_receive_inventory_blocks},
};

mod api_timer;
mod block_messages;
mod invoice;
pub mod load_server_data;
pub mod server_structs;
mod setup;
pub mod structy;
mod user_inventory_blocks;

pub struct CommsPlugin;

impl Plugin for CommsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ApiPollingTimer>()
            //.add_systems(Startup, (setup_comm, ))
            .add_systems(
                Update,
                (tick_api_receive_timer, api_receive_server_tiles)
                    .run_if(in_state(CommsApiBlockLoadState::LoadBlockData)),
            )
            .add_systems(
                Update,
                (tick_api_receive_timer, api_receive_inventory_blocks)
                    .run_if(in_state(CommsApiInventoryState::On)),
            )
            .add_systems(
                Update,
                (tick_api_receive_timer, api_receive_invoice)
                    .run_if(in_state(CommsApiState::ReceiveInvoice)),
            )
            .add_systems(
                Update,
                (
                    tick_api_receive_timer,
                    api_check_invoice,
                    api_receive_invoice_check,
                )
                    .run_if(in_state(CommsApiState::CheckInvoice)),
            )
            .add_systems(
                Update,
                (
                    api_get_server_tiles,
                    api_get_inventory_blocks,
                    api_request_invoice,
                    hide_backup_copy_btn,
                    show_backup_copy_btn,
                ),
            );
    }
}
