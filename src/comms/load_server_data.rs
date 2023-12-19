use super::api_timer::ApiPollingTimer;
use super::server_structs::GameBlocksDataFromDBMod;
use crate::eventy::RequestTileUpdates;
use crate::resourcey::{TileData, TileDataChannel, UpdateGameTimetamp};
use crate::structy::TileResource;
use crate::{CommsApiState, ServerURL, TileMap, UpdateTileTextureEvent};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use chrono::Utc;
use rand::Rng;

//pub fn load_server_data(mut commands: Commands, mut tile_map: ResMut<TileMap>) {}
//SetTileDataChannel
#[allow(unused_must_use)]
pub fn api_get_server_tiles(
    set_player_move_channel: Res<TileDataChannel>,
    api_server: Res<ServerURL>,
    mut api_state: ResMut<NextState<CommsApiState>>,
    mut gametime: ResMut<UpdateGameTimetamp>,
    mut event: EventReader<RequestTileUpdates>,
) {
    for _e in event.read() {
        info!("send api request for tiles");
        let ts_str = gametime.ts.to_string();
        //for event in player_move_event_reader.read() {
        let pool = IoTaskPool::get();
        let cc = set_player_move_channel.tx.clone();
        let server = api_server.0.to_owned();
        let _task = pool.spawn(async move {
            let api_response_text = reqwest::get(format!("{}/comms/blockdelta/{}", server, ts_str))
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            cc.try_send(api_response_text);
        });

        gametime.ts = Utc::now();

        api_state.set(CommsApiState::LoadBlockData);
    }
}

pub fn api_receive_server_tiles(
    channel: ResMut<TileDataChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_state: ResMut<NextState<CommsApiState>>,
    mut tile_map: ResMut<TileMap>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
) {
    if api_timer.timer.finished() && !channel.rx.is_empty() {
        //info!("checking for tiles response");
        let api_res = channel.rx.try_recv();
        let mut rng = rand::thread_rng();
        let mut send_update = false;
        match api_res {
            Ok(r) => {
                //info!("api_receive_server_tiles: {}", r);
                let r_invoice_result = serde_json::from_str::<GameBlocksDataFromDBMod>(&r);
                match r_invoice_result {
                    Ok(server_block_data) => {
                        //info!("{:?}", server_block_data);
                        for block_data in server_block_data.blocks {
                            let td = TileData {
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

                            tile_map.map.insert(block_data.height as u32, td);
                            send_update = true;
                        }
                        if send_update {
                            update_tile_event.send(UpdateTileTextureEvent);
                        }

                        api_state.set(CommsApiState::Off);
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
                    api_state.set(CommsApiState::Off);
                }
                e.to_string()
            }
        };
    }
}
