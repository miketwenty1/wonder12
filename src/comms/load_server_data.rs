use super::api_timer::ApiPollingTimer;
use super::server_structs::GameBlocksDataFromDBMod;
use crate::eventy::RequestTileUpdates;
use crate::resourcey::{InitGameMap, TileData, TileDataChannel, UpdateGameTimetamp};
use crate::statey::CommsApiBlockLoadState;
use crate::structy::{RequestTileType, TileResource};
use crate::{ServerURL, TileMap, UpdateTileTextureEvent};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use rand::Rng;

//pub fn load_server_data(mut commands: Commands, mut tile_map: ResMut<TileMap>) {}
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
        let pool = IoTaskPool::get();
        let cc = channel.tx.clone();
        let server = api_server.0.to_owned();
        match e.0 {
            RequestTileType::Height => {
                let _task = pool.spawn(async move {
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
                let _task = pool.spawn(async move {
                    let api_response_text =
                        reqwest::get(format!("{}/comms/blockdelta_ts/{}", server, ts_str))
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();
                    cc.try_send(api_response_text);
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
    mut tile_map: ResMut<TileMap>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
    mut gametime: ResMut<UpdateGameTimetamp>,
    mut gameinit: ResMut<InitGameMap>,
    mut get_more_tiles: EventWriter<RequestTileUpdates>,
) {
    if api_timer.timer.finished() && !channel.rx.is_empty() {
        //info!("checking for tiles response");
        let api_res = channel.rx.try_recv();
        let mut rng = rand::thread_rng();
        let mut send_update = false;
        let mut request_more_ts = false;
        let mut request_more_height = false;
        match api_res {
            Ok(r) => {
                let mut new_tile_vec = Vec::new();
                //info!("api_receive_server_tiles: {}", r);
                let r_invoice_result = serde_json::from_str::<GameBlocksDataFromDBMod>(&r);

                //info!("from the server: {:#?}", r_invoice_result);
                match r_invoice_result {
                    Ok(server_block_data) => {
                        match server_block_data {
                            GameBlocksDataFromDBMod {
                                ts_checkpoint: Some(t),
                                height_checkpoint: None,
                                blocks: _,
                            } => {
                                //info!("found timestamp checkpoint {}", t);

                                if gametime.ts == t {
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
                                //info!("found height checkpoint {}", h);

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
                            let tile_check = tile_map.map.get(&(block_data.height as u32));
                            match tile_check {
                                Some(s) => {
                                    new_td.land_index = s.land_index;
                                    if s != &new_td {
                                        new_insert_update = true;
                                        send_update = true;
                                        new_tile_vec.push(new_td.clone());

                                        //info!("old: {:#?} new: {:#?}", s, new_td);
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
                        }
                    }
                    Err(e) => {
                        info!("tile receive fail: {}", e);
                    }
                };
                r
            }
            Err(e) => {
                info!("receiving tiles: {}", e);
                if channel.rx.is_empty() {
                    api_state.set(CommsApiBlockLoadState::Off);
                }
                e.to_string()
            }
        };
    }
}
