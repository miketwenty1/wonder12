use bevy::{prelude::*, tasks::IoTaskPool};
use serde::Deserialize;

use crate::{
    eventy::{BuyBlockRequest, ClearSelectionEvent, RequestTileUpdates},
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{
        CheckInvoiceChannel, InvoiceCheckFromServer, InvoiceDataFromServer, RequestInvoiceChannel,
        TileCartVec, User,
    },
    statey::{CommsApiState, DisplayBuyUiState, ExploreState},
    structy::{GameInvoiceData, InvoiceGameBlock, RequestTileType},
    utils::convert_color_to_hexstring,
    ServerURL,
};

use super::api_timer::ApiPollingTimer;

#[derive(Debug, Clone, Deserialize, Default)]
pub enum InvoiceStatus {
    #[default]
    Pending,
    Completed,
    Expired,
    Error,
}

#[allow(unused_must_use)]
pub fn api_request_invoice(
    request_invoice_channel: Res<RequestInvoiceChannel>,
    api_server: Res<ServerURL>,
    mut button_event_reader: EventReader<BuyBlockRequest>,
    invoice_data: Res<InvoiceDataFromServer>,
    tile_cart_vec: Res<TileCartVec>,
    user: Res<User>,
    mut api_receive_state: ResMut<NextState<CommsApiState>>,
) {
    for _buy_block_data in button_event_reader.read() {
        info!("{:#?}", tile_cart_vec.vec);
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
            info!("url: {}", url);
            let _task = pool.spawn(async move {
                let api_response_text = reqwest::Client::new()
                    .post(url)
                    .header("Content-Type", "application/json")
                    .json(&b)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                cc.try_send(api_response_text);
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
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        info!("waiting to receive invoice");
        match api_res {
            Ok(r) => {
                info!("response to requesting invoice: {:#?}", r);
                let r_result = serde_json::from_str::<InvoiceDataFromServer>(&r);
                match r_result {
                    Ok(server_data) => {
                        *invoice_data = server_data;
                        qr_state.set(DisplayBuyUiState::Qr);
                        api_name_set_state.set(CommsApiState::CheckInvoice);
                        // server_event.send(ServerInvoiceIn);
                        // qr_state.set(DisplayInvoiceQr::On);
                    }
                    Err(e) => {
                        info!("response to invoice creation - fail: {}", e);
                        api_name_set_state.set(CommsApiState::Off);
                    }
                };
                r
            }
            Err(e) => {
                info!("response to invoice creation: {}", e);
                e.to_string()
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
        info!("invoice res: {:#?}", invoice_res.invoice);
        //info!("check for invoice status");

        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        let code = invoice_res.code.to_owned();
        let _task = pool.spawn(async move {
            let api_response_text = reqwest::get(format!("{}/comms/checkinvoice/{}", server, code))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            cc.try_send(api_response_text);
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
    mut qr_set_state: ResMut<NextState<DisplayBuyUiState>>,
    mut event: EventWriter<RequestTileUpdates>,
    mut invoice_data: ResMut<InvoiceDataFromServer>,
    mut clear_event: EventWriter<ClearSelectionEvent>,
    mut toast: EventWriter<ToastEvent>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        //info!("waiting to receive invoice check");
        match api_res {
            Ok(r) => {
                // info!("received something from invoice check: {}", r);
                let r_result = serde_json::from_str::<InvoiceCheckFromServer>(&r);
                match r_result {
                    Ok(o) => {
                        match o.status.as_str() {
                            "pending" => {
                                info!("pending invoice");
                            }
                            "completed" => {
                                info!("completed invoice");
                                event.send(RequestTileUpdates(RequestTileType::Ts));
                                api_name_set_state.set(CommsApiState::Off);
                                qr_set_state.set(DisplayBuyUiState::Off);
                                game_set_state.set(ExploreState::On);
                                clear_event.send(ClearSelectionEvent);
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
                        // toast.send(ToastEvent {
                        //     ttype: ToastType::Bad,
                        //     message: e.to_string(),
                        // });
                        // was getting the error receiving from empty channel error and EOF error here so commenting out toast.
                        info!("requesting check invoice fail: {}", e);
                    }
                };
                r
            }
            Err(e) => {
                //was getting the error receiving from empty channel error here so commenting out toast.
                info!("response to check invoice: {}", e);
                // toast.send(ToastEvent {
                //     ttype: ToastType::Bad,
                //     message: e.to_string(),
                // });
                e.to_string()
            }
        };
    }
}
