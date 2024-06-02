use bevy::{prelude::*, tasks::IoTaskPool};
use serde::Deserialize;

use crate::{
    async_resource_comm_channels::{CheckInvoiceChannel, RequestInvoiceChannel},
    comms::server_structs::UserGameBlock,
    eventy::{
        BuyBlockRequest, ClearSelectionEvent, HideBackupCopyBtn, ShowBackupCopyBtn,
        UpdateTilesAfterPurchase,
    },
    explore_scene::ui::inventory::event::AddInventoryRow,
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{InvoiceCheckFromServer, InvoiceDataFromServer, IsIphone, Nwc, TileCartVec, User},
    statey::{CommsApiState, DisplayBuyUiState, ExploreSelectState, ExploreState},
    structy::{ErrorMessage, GameInvoiceData, InvoiceGameBlock},
    utils::{convert_color_to_hexstring, extract_number, logout_user},
    ServerURL,
};

use super::api_timer::ApiPollingTimer;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::wasm_bindgen;

#[derive(Debug, Clone, Deserialize, Default)]
pub enum InvoiceStatus {
    #[default]
    Pending,
    Completed,
    Expired,
    Error,
}

#[allow(clippy::too_many_arguments)]
pub fn api_request_invoice(
    request_invoice_channel: Res<RequestInvoiceChannel>,
    api_server: Res<ServerURL>,
    mut button_event_reader: EventReader<BuyBlockRequest>,
    invoice_data: Res<InvoiceDataFromServer>,
    tile_cart_vec: Res<TileCartVec>,
    user: Res<User>,
    mut api_receive_state: ResMut<NextState<CommsApiState>>,
    mut toast: EventWriter<ToastEvent>,
    nwc: Res<Nwc>,
) {
    for _buy_block_data in button_event_reader.read() {
        //info!("{:#?}", tile_cart_vec.vec);
        if invoice_data.invoice.is_empty() {
            info!("requested invoice from buy button");

            let pool = IoTaskPool::get();
            let cc = request_invoice_channel.tx.clone();
            let server = api_server.0.to_owned();
            let mut block_request_block_vec = Vec::new();

            for tile in &tile_cart_vec.vec {
                // info!("tile new color {:#?}", tile.new_color);
                // info!("tile new color {:#?}", tile.new_message);
                let invoice_block = InvoiceGameBlock {
                    height: tile.height,
                    color: convert_color_to_hexstring(tile.new_color),
                    message: tile.new_message.to_string(),
                    amount: tile.cost,
                };
                info!(
                    "color before hex: {:?} after hex {:?}, and inside {:?}",
                    tile.new_color,
                    convert_color_to_hexstring(tile.new_color),
                    invoice_block.color,
                );
                block_request_block_vec.push(invoice_block);
            }

            let b = GameInvoiceData {
                blocks: block_request_block_vec,
                username: user.name.to_string(),
                refund_address: user.ln_address.to_string(),
            };
            let url = format!("{}/comms/invoice/blocks", server);
            info!("url: {}, nwc: {}", url, nwc.0);
            if nwc.0 {
                toast.send(ToastEvent {
                    ttype: ToastType::Good,
                    message: "Attempting to use Nostr Wallet Connect...".to_string(),
                });
            }

            let _task = pool.spawn(async move {
                let api_response_text_r = reqwest::Client::new()
                    .post(url)
                    .header("Content-Type", "application/json")
                    .json(&b)
                    .send()
                    .await;

                match api_response_text_r {
                    Ok(o) => {
                        let api_response_text_rr = o.text().await;

                        match api_response_text_rr {
                            Ok(o) => {
                                let _ = cc.try_send(o);
                            }
                            Err(e) => {
                                info!("error with req inv {:#?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        info!("error with requesting invoice: {:#?}", e);
                    }
                }
            });
            api_receive_state.set(CommsApiState::ReceiveInvoice);
        } else {
            info!("THIS IS ACTUALLY A BUG, report this, you shouldn't make it here blah3!");
            info!("current invoice to be paid: {:#?}", invoice_data.invoice);
        }
    }
}

pub fn api_receive_invoice(
    channel: ResMut<RequestInvoiceChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut qr_state: ResMut<NextState<DisplayBuyUiState>>,
    // mut qr_state: ResMut<NextState<RequestInvoice>>,
    // mut server_event: EventWriter<ServerInvoiceIn>,
    mut invoice_data: ResMut<InvoiceDataFromServer>,
    mut toast: EventWriter<ToastEvent>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();
        info!("waiting to receive invoice");
        match api_res {
            Ok(og_r) => {
                info!("response to requesting invoice: {:#?}", og_r);
                let r_result = serde_json::from_str::<InvoiceDataFromServer>(&og_r);
                match r_result {
                    Ok(server_data) => {
                        let invoice = server_data.invoice.clone();
                        *invoice_data = server_data.clone();
                        let temp_invoice_data = server_data;

                        let nwc = invoice_data.nwc;
                        if nwc.is_some() {
                            if nwc.unwrap() {
                                toast.send(ToastEvent {
                                    ttype: ToastType::Good,
                                    message: "Verifying Nostr Wallet Connect Payment..."
                                        .to_string(),
                                });
                            } else if !nwc.unwrap() {
                                if temp_invoice_data.error_msg.is_some() {
                                    let msg = temp_invoice_data.error_msg.unwrap();
                                    toast.send(ToastEvent {
                                        ttype: ToastType::Bad,
                                        message: msg,
                                    });
                                } else {
                                    toast.send(ToastEvent {
                                        ttype: ToastType::Bad,
                                        message: "Could not attempt Nostr Wallet Connect Payment"
                                            .to_string(),
                                    });
                                }
                                qr_state.set(DisplayBuyUiState::Qr);
                            }
                        } else {
                            qr_state.set(DisplayBuyUiState::Qr);
                        }

                        api_name_set_state.set(CommsApiState::CheckInvoice);
                        // server_event.send(ServerInvoiceIn);
                        // qr_state.set(DisplayInvoiceQr::On);

                        // trigger browser extension to pay
                        let mut event_init = web_sys::CustomEventInit::new();
                        event_init.detail(&JsValue::from_str(&invoice));
                        let event =
                            web_sys::CustomEvent::new_with_event_init_dict("weblnpay", &event_init);

                        if let Ok(o) = event {
                            if let Some(window) = web_sys::window() {
                                let _ = window.dispatch_event(&o);
                                info!("webln browser extension attempted");
                            } else {
                                info!("no attempt made for webln");
                            }
                        }
                    }
                    Err(e) => {
                        info!("{}", e);
                        if og_r.to_string().contains("invalid LN address") {
                            toast.send(ToastEvent {
                                ttype: ToastType::Bad,
                                message: "Could not validate your lightning address".to_string(),
                            });
                        } else if og_r.to_string().contains("amount for") {
                            let value =
                                serde_json::from_str::<ErrorMessage>(&og_r).unwrap_or_default();
                            let invalid_height =
                                extract_number(&value.error.to_string()).unwrap_or_default();

                            info!("{}", e);
                            toast.send(ToastEvent {
                                ttype: ToastType::Bad,
                                message: format!(
                                    "Block {} has has changed since you selected it! Reselect it or clear it!",
                                    invalid_height
                                ),
                            });
                        } else {
                            info!("{}", e);
                            toast.send(ToastEvent {
                                ttype: ToastType::Bad,
                                message: "Weird error happened maybe try logging out".to_string(),
                            });
                        }
                        info!("response to invoice creation - fail: {}", e);

                        api_name_set_state.set(CommsApiState::Off);
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
                info!("response to invoice creation: {}", e);
                //e.to_string()
            }
        };
    }
}

#[allow(unused_must_use)]
pub fn api_check_invoice(
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
                reqwest::get(format!("{}/comms/checkinvoice/{}", server, code)).await;
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
pub fn api_receive_invoice_check(
    channel: ResMut<CheckInvoiceChannel>,
    mut invoice_check_res: ResMut<InvoiceCheckFromServer>,
    api_timer: Res<ApiPollingTimer>,
    mut api_name_set_state: ResMut<NextState<CommsApiState>>,
    mut game_set_state: ResMut<NextState<ExploreState>>,
    mut game_select_set_state: ResMut<NextState<ExploreSelectState>>,
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
                                game_select_set_state.set(ExploreSelectState::On);
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
                                game_select_set_state.set(ExploreSelectState::On);
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
                                game_select_set_state.set(ExploreSelectState::On);
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
                                game_select_set_state.set(ExploreSelectState::On);
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

#[allow(clippy::too_many_arguments)]
pub fn show_backup_copy_btn(
    mut event: EventReader<ShowBackupCopyBtn>,
    invoice: Res<InvoiceDataFromServer>,
) {
    for _e in event.read() {
        let invoice = &invoice.invoice;
        let window: web_sys::Window = web_sys::window().expect("window");
        let document_text = window.document().unwrap().get_element_by_id("textToCopy");
        let document_btn = window.document().unwrap().get_element_by_id("copyButton");

        // let _task = spawn_local(async move {
        match document_text {
            Some(o) => {
                let vall = o.dyn_into::<web_sys::HtmlElement>();
                match vall {
                    Ok(o) => {
                        info!("worked for html element {:#?}", o);
                        // let cleaninvoice = invoice
                        //     .strip_prefix("lightning:")
                        //     .unwrap_or_else(|| invoice);
                        o.set_inner_text(invoice);
                        match document_btn {
                            Some(o) => {
                                info!("btn found");

                                let btn_val = o.dyn_into::<web_sys::HtmlElement>();

                                match btn_val {
                                    Ok(o) => {
                                        info!("messed up");
                                        let st = o.style().set_property("visibility", "visible");

                                        match st {
                                            Ok(_) => {
                                                info!("style set it seems");
                                            }
                                            Err(e) => {
                                                info!("error for style {:#?}", e);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        info!("messed up {:#?}", e);
                                    }
                                }
                            }
                            None => {
                                info!("btn not found wtf");
                            }
                        }
                    }
                    Err(e) => {
                        info!("error {:#?}", e);
                    }
                }
            }
            None => {
                info!("no p found");
            }
        };
    }
}

#[allow(clippy::too_many_arguments)]
pub fn hide_backup_copy_btn(mut event: EventReader<HideBackupCopyBtn>) {
    for _e in event.read() {
        let window: web_sys::Window = web_sys::window().expect("window");
        let document_btn = window.document().unwrap().get_element_by_id("copyButton");

        match document_btn {
            Some(o) => {
                let btn_val = o.dyn_into::<web_sys::HtmlElement>();

                match btn_val {
                    Ok(o) => {
                        info!("messed up");
                        let st = o.style().set_property("visibility", "hidden");

                        match st {
                            Ok(_) => {
                                info!("style set it seems to hidden");
                            }
                            Err(e) => {
                                info!("error for style {:#?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        info!("messed up {:#?}", e);
                    }
                }
            }
            None => {
                info!("iOS style copy button failure");
            }
        }
    }
}
