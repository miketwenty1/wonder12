use bevy::{prelude::*, tasks::IoTaskPool};
use serde::Deserialize;

use crate::{
    async_resource_comm_channels::{CheckInvoiceChannel, RequestInvoiceChannel},
    comms::server_structs::UserGameBlock,
    eventy::{
        BuyBlockRequest, ClearSelectionEvent, HideBackupCopyBtn, ShowBackupCopyBtn,
        UpdateTilesAfterPurchase,
    },
    overlay_ui::{
        inventory::event::AddInventoryRow,
        toast::{ToastEvent, ToastType},
    },
    resourcey::{InvoiceCheckFromServer, InvoiceDataFromServer, IsIphone, TileCartVec, User},
    statey::{CommsApiState, DisplayBuyUiState, ExploreState},
    structy::{ErrorMessage, GameInvoiceData, InvoiceGameBlock},
    utils::{convert_color_to_hexstring, extract_number, logout_user},
    ServerURL,
};

use super::api_timer::ApiPollingTimer;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::wasm_bindgen;

#[allow(unused_must_use)]
pub fn api_get_messages_for_block(
    channel: Res<CheckInvoiceChannel>,
    api_server: Res<ServerURL>,
    api_timer: Res<ApiPollingTimer>,
    invoice_res: Res<InvoiceDataFromServer>,
    //mut details_button_event_reader: EventReader<BlockDetailClick>,
) {
    if api_timer.timer.finished() {
        //info!("invoice res: {:#?}", invoice_res.invoice);
        info!("check for invoice status");

        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        let code = invoice_res.code.to_owned();
        let _task = pool.spawn(async move {
            let api_response_r =
                reqwest::get(format!("{}/comms/blockmessages/{}", server, code)).await;
            match api_response_r {
                Ok(o) => {
                    let api_response_text_r = o.text().await;
                    match api_response_text_r {
                        Ok(o) => {
                            cc.try_send(o);
                        }
                        Err(e) => {
                            info!("failed to parse to check invoice to text {:#?}", e);
                            cc.try_send(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    info!("failed to receive a check invoice {:#?}", e);
                    cc.try_send(e.to_string());
                }
            }
        });
    }
}

#[allow(clippy::too_many_arguments)]
pub fn api_receive_messages_for_block(
    channel: ResMut<CheckInvoiceChannel>,
    mut invoice_check_res: ResMut<InvoiceCheckFromServer>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut game_set_state: ResMut<NextState<ExploreState>>,
    mut qr_set_state: ResMut<NextState<DisplayBuyUiState>>,
    mut invoice_data: ResMut<InvoiceDataFromServer>,
    mut clear_event: EventWriter<ClearSelectionEvent>,
    mut toast: EventWriter<ToastEvent>,
    mut bkp_clipboard_btn: EventWriter<HideBackupCopyBtn>,
    iphone: Res<IsIphone>,
    mut inv_event: EventWriter<AddInventoryRow>,
    mut update_tiles_e: EventWriter<UpdateTilesAfterPurchase>,
    tile_cart_vec: Res<TileCartVec>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        //info!("waiting to receive invoice check");
        match api_res {
            Ok(og_r) => {
                let r_result = serde_json::from_str::<InvoiceCheckFromServer>(&og_r);
                match r_result {
                    Ok(o) => {
                        match o.status.as_str() {
                            "pending" => {
                                info!("pending invoice");
                            }
                            "completed" => {
                                info!("completed invoice");
                                //event.send(RequestTileUpdates(RequestTileType::Ts));
                                update_tiles_e.send(UpdateTilesAfterPurchase);
                                let mut inv = Vec::new();
                                for tile in &tile_cart_vec.vec {
                                    let user_game_block = UserGameBlock {
                                        height: tile.height,
                                        amount: tile.cost,
                                        color: convert_color_to_hexstring(tile.new_color),
                                    };
                                    inv.push(user_game_block);
                                }
                                inv_event.send(AddInventoryRow(inv));
                                api_name_set_state.set(CommsApiState::Off);
                                qr_set_state.set(DisplayBuyUiState::Off);
                                game_set_state.set(ExploreState::On);
                                clear_event.send(ClearSelectionEvent);
                                if iphone.0 {
                                    bkp_clipboard_btn.send(HideBackupCopyBtn);
                                }

                                toast.send(ToastEvent {
                                    ttype: ToastType::Good,
                                    message: "Payment Completed!".to_string(),
                                });
                                *invoice_data = InvoiceDataFromServer {
                                    ..Default::default()
                                };
                            }
                            "expired" => {
                                info!("expired invoice");
                                api_name_set_state.set(CommsApiState::Off);
                                qr_set_state.set(DisplayBuyUiState::Off);
                                game_set_state.set(ExploreState::On);
                                if iphone.0 {
                                    bkp_clipboard_btn.send(HideBackupCopyBtn);
                                }
                                toast.send(ToastEvent {
                                    ttype: ToastType::Bad,
                                    message: "Payment Expired!".to_string(),
                                });
                                *invoice_data = InvoiceDataFromServer {
                                    ..Default::default()
                                };
                            }
                            "error" => {
                                info!("error invoice");
                                api_name_set_state.set(CommsApiState::Off);
                                qr_set_state.set(DisplayBuyUiState::Off);
                                game_set_state.set(ExploreState::On);
                                toast.send(ToastEvent {
                                    ttype: ToastType::Bad,
                                    message: "Error002".to_string(),
                                });
                            }
                            _ => {
                                info!("Something very bizaare happened picka2");
                                api_name_set_state.set(CommsApiState::Off);
                                qr_set_state.set(DisplayBuyUiState::Off);
                                game_set_state.set(ExploreState::On);
                                toast.send(ToastEvent {
                                    ttype: ToastType::Bad,
                                    message: "Error001".to_string(),
                                });
                            }
                        }
                        *invoice_check_res = o;
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
                        }
                        info!("requesting check invoice fail: {}", e);
                    }
                };
                //r
            }
            Err(e) => {
                if !e.to_string().contains("EOF") && !e.to_string().contains("empty channel") {
                    toast.send(ToastEvent {
                        ttype: ToastType::Bad,
                        message: e.to_string(),
                    });
                }
                info!("response to check invoice: {}", e);

                //e.to_string()
            }
        };
    }
}
