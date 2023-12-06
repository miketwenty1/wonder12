use super::api_timer::ApiPollingTimer;
use super::server_structs::GameBlocksDataFromDBMod;
use crate::resourcey::{TileData, TileDataChannel};
use crate::structy::TileResource;
use crate::{CommsState, ServerURL, TileMap, UpdateTileTextureEvent};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

//pub fn load_server_data(mut commands: Commands, mut tile_map: ResMut<TileMap>) {}
//SetTileDataChannel
#[allow(unused_must_use)]
pub fn api_get_server_tiles(
    set_player_move_channel: Res<TileDataChannel>,
    api_server: Res<ServerURL>,
    mut api_state: ResMut<NextState<CommsState>>,
    // mut player_move_event_reader: EventReader<PlayerMove>,
) {
    info!("send api request for tiles");
    //for event in player_move_event_reader.read() {
    let pool = IoTaskPool::get();
    let cc = set_player_move_channel.tx.clone();
    let server = api_server.0.to_owned();
    let _task = pool.spawn(async move {
        let api_response_text = reqwest::get(format!("{}/comms/blockdelta", server))
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        cc.try_send(api_response_text);
    });

    api_state.set(CommsState::On);
    //}
}

pub fn api_receive_server_tiles(
    channel: ResMut<TileDataChannel>,
    api_timer: Res<ApiPollingTimer>,
    mut api_state: ResMut<NextState<CommsState>>,
    mut tile_map: ResMut<TileMap>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
) {
    if api_timer.timer.finished() && !channel.rx.is_empty() {
        //info!("checking for tiles response");
        let api_res = channel.rx.try_recv();

        let mut send_update = false;
        match api_res {
            Ok(r) => {
                //info!("response to move player: {}", r);
                let r_invoice_result = serde_json::from_str::<GameBlocksDataFromDBMod>(&r);
                match r_invoice_result {
                    Ok(server_block_data) => {
                        //info!("{:?}", server_block_data);
                        for block_data in server_block_data.blocks {
                            let td = TileData {
                                ln_address: block_data.refund_ln_addr,
                                owner: block_data.username,
                                color: Color::hex(block_data.color).unwrap(),
                                message: block_data.message,
                                resource: TileResource::Wheat,
                                amount: block_data.amount as u32,
                                hash: "".to_string(),
                                height: block_data.height as u32,
                            };

                            tile_map.map.insert(block_data.height as u32, td);
                            send_update = true;
                        }
                        if send_update {
                            update_tile_event.send(UpdateTileTextureEvent);
                        }

                        api_state.set(CommsState::Off);
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
                    api_state.set(CommsState::Off);
                }
                e.to_string()
            }
        };
    }
}
