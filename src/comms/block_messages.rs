use bevy::{prelude::*, tasks::IoTaskPool};

use crate::{
    async_resource_comm_channels::BlockMessagesStorageChannel,
    eventy::{BlockDetailMessage, MessageReceivedFromServer},
    explore_scene::overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{TileCartVec, UserPurchasedBlockMessage},
    utils::logout_user,
    ServerURL,
};

use super::{api_timer::ApiPollingTimer, structy::MessagesFromServer};

#[allow(dead_code)]
pub fn api_get_messages_for_block(
    channel: Res<BlockMessagesStorageChannel>,
    api_server: Res<ServerURL>,
    mut message_reader: EventReader<BlockDetailMessage>,
) {
    for eheight in message_reader.read() {
        //info!("invoice res: {:#?}", invoice_res.invoice);
        info!("check for invoice status");

        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        let height = eheight.0;
        let _task = pool.spawn(async move {
            let api_response_r =
                reqwest::get(format!("{}/comms/blockmessages/{}", server, height)).await;
            match api_response_r {
                Ok(o) => {
                    let api_response_text_r = o.text().await;
                    match api_response_text_r {
                        Ok(o) => {
                            let _ = cc.try_send(o);
                        }
                        Err(e) => {
                            info!("failed to parse to message text: {:#?}", e);
                            let _ = cc.try_send(e.to_string());
                        }
                    }
                }
                Err(e) => {
                    info!("failed to send in channel message: {:#?}", e);
                    let _ = cc.try_send(e.to_string());
                }
            }
        });
    }
}

#[allow(clippy::too_many_arguments, dead_code)]
pub fn api_receive_messages_for_block(
    channel: Res<BlockMessagesStorageChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut toast: EventWriter<ToastEvent>,
    mut tile_cart_vec: ResMut<TileCartVec>,
    mut messages_received: EventWriter<MessageReceivedFromServer>,
) {
    if api_timer.timer.finished() {
        let api_res = channel.rx.try_recv();

        //info!("waiting to receive invoice check");
        match api_res {
            Ok(og_r) => {
                let r_result = serde_json::from_str::<MessagesFromServer>(&og_r);
                match r_result {
                    Ok(o) => {
                        info!("here are messages: {:#?}", o);
                        messages_received.send(MessageReceivedFromServer(o.height));
                        // the reason for using this index instead of the index inside the tile cart vec is to avoid
                        // a hypotheical situation where the index and the height aren't the same due to a race
                        // condition.. the user may toggle off the block that is being received
                        for (index, tile) in tile_cart_vec.clone().vec.iter().enumerate() {
                            if tile.height == o.height {
                                let mut incoming_messages = vec![];
                                for message in &o.messages {
                                    let pmessage = UserPurchasedBlockMessage {
                                        username: message.username.clone(),
                                        value: message.amount as u32,
                                        message: message.message.clone(),
                                    };
                                    incoming_messages.push(pmessage);
                                }
                                tile_cart_vec.vec[index].messages = Some(incoming_messages);
                            }
                        }
                    }
                    Err(e) => {
                        if og_r.to_string().contains("logout") {
                            logout_user("messages call");
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
            }
            Err(e) => {
                if !e.to_string().contains("EOF") && !e.to_string().contains("empty channel") {
                    toast.send(ToastEvent {
                        ttype: ToastType::Bad,
                        message: e.to_string(),
                    });
                }
                if !e.to_string().contains("empty channel") {
                    info!("response to message call: {}", e);
                }
            }
        };
    }
}
