use super::api_timer::ApiPollingTimer;
use crate::eventy::RequestInventoryEvent;
use crate::overlay_ui::inventory::state::InventoryUiState;
use crate::overlay_ui::toast::{ToastEvent, ToastType};
use crate::resourcey::{InventoryBlocks, UserBlockInventory};
use crate::statey::CommsApiInventoryState;
use crate::utils::logout_user;
use crate::ServerURL;
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

#[allow(unused_must_use)]
pub fn api_get_inventory_blocks(
    channel: Res<UserBlockInventory>,
    api_server: Res<ServerURL>,
    mut event: EventReader<RequestInventoryEvent>,
    mut api_inventory_state: ResMut<NextState<CommsApiInventoryState>>,
) {
    for _e in event.read() {
        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        let _task = pool.spawn(async move {
            let api_response_r = reqwest::get(format!("{}/comms/currentuserblocks", server)).await;
            match api_response_r {
                Ok(o) => {
                    let api_response_text_r = o.text().await;
                    match api_response_text_r {
                        Ok(o) => {
                            cc.try_send(o);
                        }
                        Err(e) => {
                            info!("failed to parse inventory text {:#?}", e);
                            cc.try_send(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    info!("failed to receive inventory {:#?}", e);
                    cc.try_send(e.to_string());
                }
            }
        });
        api_inventory_state.set(CommsApiInventoryState::On);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn api_receive_inventory_blocks(
    channel: Res<UserBlockInventory>,
    mut data_res: ResMut<InventoryBlocks>,
    api_timer: Res<ApiPollingTimer>,
    mut api_inventory_state: ResMut<NextState<CommsApiInventoryState>>,
    mut toast: EventWriter<ToastEvent>,
    mut inventory_ui_state: ResMut<NextState<InventoryUiState>>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        //info!("waiting to receive invoice check");
        match api_res {
            Ok(og_r) => {
                let r_result = serde_json::from_str::<InventoryBlocks>(&og_r);
                match r_result {
                    Ok(o) => {
                        info!("receiving inventory: {:#?}", o);
                        *data_res = o;
                        api_inventory_state.set(CommsApiInventoryState::Off);
                        inventory_ui_state.set(InventoryUiState::On);
                    }
                    Err(e) => {
                        if og_r.to_string().contains("logout") {
                            logout_user("invoice check 1");
                        } else if !e.to_string().contains("EOF")
                            && !e.to_string().contains("empty channel")
                        {
                            toast.send(ToastEvent {
                                ttype: ToastType::Bad,
                                message: e.to_string(),
                            });
                            api_inventory_state.set(CommsApiInventoryState::Off);
                        }
                        info!("requesting check invoice fail: {}", e);
                    }
                };
            }
            Err(e) => {
                if !e.to_string().contains("EOF") && !e.to_string().contains("empty channel") {
                    toast.send(ToastEvent {
                        ttype: ToastType::Bad,
                        message: e.to_string(),
                    });
                }
                info!("response to check invoice: {}", e);
            }
        };
    }
}
