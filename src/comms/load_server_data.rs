use super::api_timer::ApiPollingTimer;
use super::server_structs::GameBlocksDataFromDBMod;
use crate::comms::server_structs::UserGameBlock;
use crate::eventy::{DespawnInventoryHeights, RequestTileUpdates};
use crate::overlay_ui::inventory::event::AddInventoryRow;
use crate::overlay_ui::toast::{ToastEvent, ToastType};
use crate::resourcey::{
    InitGameMap, TileData, TileDataChannel, UpdateGameTimetamp, UserInventoryBlocks,
};
use crate::statey::CommsApiBlockLoadState;
use crate::structy::{RequestTileType, TileResource};
use crate::utils::{convert_color_to_hexstring, logout_user};
use crate::{ServerURL, UpdateTileTextureEvent, WorldOwnedTileMap};
use bevy::prelude::*;
//use bevy::tasks::IoTaskPool;
use rand::Rng;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

//SetTileDataChannel
#[allow(unused_must_use)]
pub fn api_get_server_tiles(
    channel: Res<TileDataChannel>,
    api_server: Res<ServerURL>,
    mut api_load_block_state: ResMut<NextState<CommsApiBlockLoadState>>,
    gametime: Res<UpdateGameTimetamp>,
    init: Res<InitGameMap>,
    mut event: EventReader<RequestTileUpdates>,
) {
    for e in event.read() {
        //info!("send api request for tiles");
        let ts_str = gametime.ts.to_string();
        let height_str = init.height.to_string();
        //for event in player_move_event_reader.read() {
        //let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        match e.0 {
            RequestTileType::Height => {
                info!("get height tiles sending {}", height_str);
                spawn_local(async move {
                    let api_response_text =
                        reqwest::get(format!("{}/comms/blockdelta_height/{}", server, height_str))
                            .await;
                    match api_response_text {
                        Ok(o) => {
                            let inner = o.text().await;
                            match inner {
                                Ok(o_inner) => {
                                    cc.try_send(o_inner);
                                }
                                Err(e) => info!("inner error blockdelta_height {}", e),
                            }
                        }
                        Err(e) => info!("error for blockdelta_height {}", e),
                    }
                });
            }
            RequestTileType::Ts => {
                info!("get ts tiles sending {}", ts_str);
                spawn_local(async move {
                    let api_response_r =
                        reqwest::get(format!("{}/comms/blockdelta_ts/{}", server, ts_str)).await;

                    match api_response_r {
                        Ok(o) => {
                            let api_response_text_r = o.text().await;

                            match api_response_text_r {
                                Ok(o) => {
                                    cc.try_send(o);
                                }
                                Err(e) => {
                                    info!("error for request tile ts {:#?}", e);
                                    cc.try_send(e.to_string());
                                }
                            }
                        }
                        Err(e) => {
                            info!("error for request tile ts {:#?}", e);
                            cc.try_send(e.to_string());
                        }
                    }
                });
            }
        }

        //gametime.ts = Utc::now();

        api_load_block_state.set(CommsApiBlockLoadState::LoadBlockData);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn api_receive_server_tiles(
    channel: ResMut<TileDataChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_state: ResMut<NextState<CommsApiBlockLoadState>>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
    mut gametime: ResMut<UpdateGameTimetamp>,
    mut gameinit: ResMut<InitGameMap>,
    mut get_more_tiles: EventWriter<RequestTileUpdates>,
    mut toast: EventWriter<ToastEvent>,
    mut despawn_inventory: EventWriter<DespawnInventoryHeights>,
    mut spawn_inventory: EventWriter<AddInventoryRow>,
    inventory: Res<UserInventoryBlocks>,
) {
    if api_timer.timer.finished() && !channel.rx.is_empty() {
        //info!("checking for tiles response");
        let api_res = channel.rx.try_recv();
        let mut rng = rand::thread_rng();
        let mut send_update = false;
        let mut request_more_ts = false;
        let mut request_more_height = false;
        // inventory remove vec
        let mut remove_inventory_holder: Vec<u32> = Vec::new();
        let mut add_inventory_holder: Vec<UserGameBlock> = Vec::new();
        match api_res {
            Ok(og_r) => {
                let mut new_tile_vec = Vec::new();
                // for getting a vec of the inventory items needing to be despawned out of inventory of a user.
                // if a tile comes in and the previous owner is the user.. (AND the new owner isn't the user) add to vec.

                //info!("api_receive_server_tiles: {}", r);
                let r_block_result = serde_json::from_str::<GameBlocksDataFromDBMod>(&og_r);

                match r_block_result {
                    Ok(server_block_data) => {
                        match server_block_data.clone() {
                            GameBlocksDataFromDBMod {
                                ts_checkpoint: Some(t),
                                height_checkpoint: None,
                                blocks: _,
                            } => {
                                info!("received timestamp checkpoint {}", t);

                                if gametime.ts >= t {
                                    //info!("==");
                                } else {
                                    request_more_ts = true;
                                    info!("receiving tiles gamet{}, servert{}", gametime.ts, t);
                                }
                                gametime.ts = t;
                            }
                            GameBlocksDataFromDBMod {
                                ts_checkpoint: None,
                                height_checkpoint: Some(h),
                                blocks: _,
                            } => {
                                info!("received height checkpoint {}", h);

                                if gameinit.height == h {
                                    //info!("==");
                                } else {
                                    request_more_height = true;
                                    //info!("request more height");
                                }
                                gameinit.height = h;
                            }
                            _ => println!("Invalid state or both are None"),
                        }

                        for block_data in server_block_data.blocks {
                            let mut new_insert_update = false;
                            let mut new_td = TileData {
                                ln_address: block_data.refund_ln_addr,
                                username: block_data.username,
                                color: Color::hex(block_data.color).unwrap(),
                                message: block_data.message,
                                resource: TileResource::Wheat,
                                value: block_data.amount as u32,
                                cost: (block_data.amount * 2) as u32,
                                hash: "".to_string(),
                                height: block_data.height as u32,
                                land_index: rng.gen_range(1..=11),
                                event_date: block_data.event_date,
                            };

                            // // // // inventory update code

                            let user_inv_map = &inventory.ownedblocks;
                            let inv_o = user_inv_map.get(&new_td.height);
                            if let Some(_s) = inv_o {
                                let inv_amount = user_inv_map.get(&new_td.height).unwrap().amount;

                                // shouldnt need this!!!
                                // let checker_inv_value = if inv_amount == 0 { 128 } else { inv_amount * 2 };
                                info!(
                                    "PRE!!! {}, invamount: {}, checkamount: {}",
                                    new_td.height, inv_amount, new_td.value
                                );
                                //let aa = new_td.clone();
                                #[allow(clippy::comparison_chain)]
                                if user_inv_map.contains_key(&new_td.height) {
                                    if inv_amount < new_td.value {
                                        info!("need to DEL this from inventory: {}, invamount: {}, checkamount: {}", new_td.height, inv_amount, new_td.value);
                                        remove_inventory_holder.push(new_td.height);
                                    } else if inv_amount > new_td.value {
                                        info!("need to ADD this from inventory: {}, invamount: {}, checkamount: {}", new_td.height, inv_amount, new_td.value);
                                        add_inventory_holder.push(UserGameBlock {
                                            height: new_td.height,
                                            amount: new_td.value,
                                            color: convert_color_to_hexstring(new_td.color),
                                        });
                                    } else {
                                        info!("block came in and matches inv amount");
                                    }
                                }
                            }

                            //update_inventory(new_td);
                            let tile_check = tile_map.map.get(&(block_data.height as u32));
                            match tile_check {
                                Some(s) => {
                                    new_td.land_index = s.land_index;
                                    if s != &new_td {
                                        new_insert_update = true;
                                        send_update = true;
                                        new_tile_vec.push(new_td.clone());
                                    }
                                }
                                None => {
                                    new_insert_update = true;
                                    send_update = true;
                                    new_tile_vec.push(new_td.clone());
                                }
                            }
                            if new_insert_update {
                                tile_map.map.insert(block_data.height as u32, new_td);
                            }
                        }
                        // // // inventory update code
                        if !remove_inventory_holder.is_empty() {
                            despawn_inventory
                                .send(DespawnInventoryHeights(remove_inventory_holder));
                        }
                        if !add_inventory_holder.is_empty() {
                            spawn_inventory.send(AddInventoryRow(add_inventory_holder));
                        }
                        // // // inventory update code

                        if send_update {
                            update_tile_event.send(UpdateTileTextureEvent(new_tile_vec));

                            if request_more_height {
                                info!("requesting more height based tiles");
                                get_more_tiles.send(RequestTileUpdates(RequestTileType::Height));
                            } else if request_more_ts {
                                info!("requesting more time based tiles");
                                get_more_tiles.send(RequestTileUpdates(RequestTileType::Ts));
                            } else {
                                api_state.set(CommsApiBlockLoadState::Off);
                            }
                        } else {
                            api_state.set(CommsApiBlockLoadState::Off);
                            // todo ad
                        }
                    }
                    Err(e) => {
                        if og_r.to_string().contains("logout") {
                            logout_user("receive server tiles 1");
                        } else if !e.to_string().contains("EOF")
                            && !e.to_string().contains("empty channel")
                        {
                            if e.to_string().contains("line 1 column 1") {
                                toast.send(ToastEvent {
                                    ttype: ToastType::Bad,
                                    message: "Seems you lost connection to the server".to_string(),
                                });
                            } else {
                                toast.send(ToastEvent {
                                    ttype: ToastType::Bad,
                                    message: format!("error: {}", e),
                                });
                            }
                        }
                        info!("tile receive fail: {}", e);
                    }
                };
                //og_r
            }
            Err(e) => {
                info!("receiving tiles: {}", e);
                if !e.to_string().contains("EOF") && !e.to_string().contains("empty channel") {
                    toast.send(ToastEvent {
                        ttype: ToastType::Bad,
                        message: e.to_string(),
                    });
                }
                if channel.rx.is_empty() {
                    api_state.set(CommsApiBlockLoadState::Off);
                }
                //e.to_string()
            }
        };
    }
}
