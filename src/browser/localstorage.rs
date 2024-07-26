use crate::async_resource_comm_channels::{
    BrowserCheckpointLocalStorageChannel, BrowserIndexedDBStorageChannel,
    BrowserMapLocalStorageChannel,
};
use crate::browser::state::BrowserIndexedDBStorageState;
use crate::comms::structy::{TrimExplorerTileVec, TrimTileLocalBrowserStorage};
use crate::consty::DEFAULT_HASH;
use crate::eventy::{RequestTileUpdates, UpdateTileTextureEvent};
use crate::resourcey::{BlockExplorer, CheckpointTimetamp, TileData};
use crate::resourcey::{UpdateGameTimetamp, WorldOwnedTileMap};
use crate::structy::{RequestTileType, TileResource};
use crate::utils::{calculate_index_for_resourced_lands, get_land_index, get_resource_for_tile};
use bevy::prelude::*;

use chrono::{NaiveDateTime, Timelike, Utc};
use serde_json::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::{js_sys, wasm_bindgen};

use super::event::{ReadIndexedDBStorage, ReadLocalBrowserStorage, WriteLocalBrowserStorage};
use super::resource::BrowserPollingTimer;
use super::state::BrowserLocalStorageState;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_wasm_bindgen::from_value as wasm_from_value;
use wasm_bindgen::prelude::*;

pub fn write_local_storage(
    mut event: EventReader<WriteLocalBrowserStorage>,
    tile_map: Res<WorldOwnedTileMap>,
    gametime: Res<UpdateGameTimetamp>,
) {
    for _e in event.read() {
        let mut event_map = web_sys::CustomEventInit::new();
        let mut event_ts = web_sys::CustomEventInit::new();
        let trim_browser_tile = tile_map.trim_for_browser_storage();
        let www = serde_json::to_string(&trim_browser_tile).expect("world map is a string");
        let map_json_val = &JsValue::from_str(&www);
        let ts_json_val = &JsValue::from_str(&gametime.ts.to_string());
        info!("ts to be inserted {:#?}", ts_json_val);
        event_map.detail(map_json_val);
        event_ts.detail(ts_json_val);
        let mapdata_event =
            web_sys::CustomEvent::new_with_event_init_dict("localbrowserstorage", &event_map);
        let ts_event = web_sys::CustomEvent::new_with_event_init_dict("mapcheckpoint", &event_ts);

        if let Ok(o) = ts_event {
            if let Some(window) = web_sys::window() {
                let _ = window.dispatch_event(&o);
                info!("localbrowserstorage attempted");
            } else {
                info!("localbrowserstorage else");
            }
        }

        if let Ok(o) = mapdata_event {
            if let Some(window) = web_sys::window() {
                let _ = window.dispatch_event(&o);
                info!("localbrowserstorage attempted");
            } else {
                info!("localbrowserstorage else");
            }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn retrieveLocalBrowserGameData() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window)]
    fn retrieveIndexedDBBlockExplorerData() -> js_sys::Promise;

    #[wasm_bindgen(js_namespace = window)]
    fn retrieveCheckpoint() -> js_sys::Promise;
}

// this function will grab the worldmap and checkpoint from localbrowser storage and put both of those items into a channel to be consumed.
pub fn request_local_storage(
    mut event: EventReader<ReadLocalBrowserStorage>,
    map_channel: Res<BrowserMapLocalStorageChannel>,
    checkpoint_channel: Res<BrowserCheckpointLocalStorageChannel>,
) {
    for _e in event.read() {
        info!("did this happen1");
        let map_cc = map_channel.tx.clone();
        let checkpoint_cc = checkpoint_channel.tx.clone();
        spawn_local(async move {
            let mapdata_promise = retrieveLocalBrowserGameData();
            let checkpoint_promise = retrieveCheckpoint();
            let checkpoint_result = JsFuture::from(checkpoint_promise).await;
            let mapdata_result = JsFuture::from(mapdata_promise).await;

            match checkpoint_result {
                Ok(o) => {
                    info!("did this happen1-1");
                    // info!("good checkpoint {:#?}", o);
                    let _ = checkpoint_cc.try_send(o.as_string().unwrap_or_default());
                }
                Err(e) => {
                    info!("error from checkpoint local browser storage {:#?}", e);
                    let _ = checkpoint_cc.try_send("errorornotfound".to_string());
                }
            }
            match mapdata_result {
                Ok(o) => {
                    info!("did this happen1-2");
                    let _ = map_cc.try_send(o.as_string().unwrap_or_default());
                }
                Err(e) => {
                    info!("error from mapdata local browser storage {:#?}", e);
                    let _ = map_cc.try_send("errorornotfound".to_string());
                }
            }
        });
        info!("read local storage");
    }
}

// this will consume from the 2 channels above on the internal of the polling timer
#[allow(clippy::too_many_arguments)]
pub fn readcheck_local_storage(
    map_channel: Res<BrowserMapLocalStorageChannel>,
    checkpoint_channel: Res<BrowserCheckpointLocalStorageChannel>,
    browser_poll_timer: Res<BrowserPollingTimer>,
    mut request_tiles_event: EventWriter<RequestTileUpdates>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
    mut browser_state: ResMut<NextState<BrowserLocalStorageState>>,
    mut game_time: ResMut<UpdateGameTimetamp>,
    mut checkpoint_time: ResMut<CheckpointTimetamp>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
    mut indexed_state: ResMut<NextState<BrowserIndexedDBStorageState>>,
    indexed_res_bool: Res<BlockExplorer>,
) {
    if browser_poll_timer.timer.just_finished() {
        info!("did this happen2");
        info!("ticky boy");
        let map_res = map_channel.rx.try_recv();
        let checkpoint_res = checkpoint_channel.rx.try_recv();

        match map_res {
            Ok(o) => {
                // do something here if the data exist
                //info!("checkpoint_res: {:#?}, map_res: {:#?}", checkpoint_res, o);
                if o == "errorornotfound" {
                    request_tiles_event.send(RequestTileUpdates(RequestTileType::Height));
                } else {
                    let r_result = serde_json::from_str::<TrimTileLocalBrowserStorage>(&o);
                    match r_result {
                        Ok(o) => {
                            let world_map_converted = o.convert_trim_to_tilemap();

                            match checkpoint_res {
                                Ok(o) => {
                                    // info!("this is the string for the date: {}", o);
                                    let format = "%Y-%m-%d %H:%M:%S.%3f %Z";
                                    info!("checkpoint string read is: {}", o);
                                    //2024-02-10 06:44:34.499 UTC
                                    let datetime_utc = NaiveDateTime::parse_from_str(&o, format);

                                    match datetime_utc {
                                        Ok(o) => {
                                            let nanos = o.nanosecond(); // Get current nanoseconds
                                            let millis_only_nanos = (nanos / 1_000_000) * 1_000_000; // Convert to only milliseconds nanoseconds

                                            let now_with_millis_only =
                                                o.with_nanosecond(millis_only_nanos).unwrap(); // Safe to unwrap here

                                            info!(
                                                "millis_only_nanos: {} - non_with_milis_only: {}",
                                                millis_only_nanos, now_with_millis_only
                                            );
                                            // using nativeDateTime adds microseconds
                                            let datetime_utc_nomicro =
                                                now_with_millis_only.and_utc(); //.with_nanosecond(0).unwrap();
                                            info!("before: {}, after {}", o, datetime_utc_nomicro);
                                            game_time.ts = datetime_utc_nomicro;
                                            checkpoint_time.ts = datetime_utc_nomicro;
                                            *tile_map = world_map_converted.clone();
                                            browser_state.set(BrowserLocalStorageState::Off);

                                            // now trigger BrowserIndexedDBStorageState after completing the read with local browser storage.
                                            if indexed_res_bool.0 {
                                                info!("did this go on?");
                                                indexed_state.set(BrowserIndexedDBStorageState::On);
                                            } else {
                                                info!("it didn't work?");
                                            }

                                            request_tiles_event
                                                .send(RequestTileUpdates(RequestTileType::Ts));

                                            let tiles = world_map_converted.to_tiledata_vec();

                                            update_tile_event.send(UpdateTileTextureEvent(tiles));
                                        }
                                        Err(e) => {
                                            info!("oh no browser pull 2, {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    info!("oh no browser pull 3, {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            info!("some error with parsing browser storage {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                info!("probably don't have any browser storage, if this is a fresh session ignore this, otherwise: {}", e);
                browser_state.set(BrowserLocalStorageState::Off);
            }
        }
    }
}

pub fn convert_js_value_to_string(js_value: JsValue) -> Result<String, JsValue> {
    let value: Value = wasm_from_value(js_value)
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize: {:?}", e)))?;
    let stringified = serde_json::to_string(&value)
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize: {:?}", e)))?;
    Ok(stringified)
}

pub fn request_indexeddb_storage(indexeddb_channel: Res<BrowserIndexedDBStorageChannel>) {
    info!("did this happen3");
    let indexeddb_cc = indexeddb_channel.tx.clone();
    spawn_local(async move {
        let indexed_promise = retrieveIndexedDBBlockExplorerData();
        let indexed_result = JsFuture::from(indexed_promise).await;

        match indexed_result {
            Ok(o) => {
                info!("what is this JsValue?: {:#?}", o);
                let ooo = convert_js_value_to_string(o);
                match ooo {
                    Ok(o) => {
                        info!("str parse?: {:#?}", o);
                        let _ = indexeddb_cc.try_send(o);
                    }
                    Err(e) => {
                        info!("error with convert_js_value_to_string(), {:#?}", e);
                    }
                }
            }
            Err(e) => {
                info!("error from checkpoint local browser storage {:#?}", e);
                let _ = indexeddb_cc.try_send("errorornotfound".to_string());
            }
        }
    });
    info!("read local storage");
}

pub fn readcheck_indexeddb_storage(
    comms_channel: Res<BrowserIndexedDBStorageChannel>,
    //checkpoint_channel: Res<BrowserCheckpointLocalStorageChannel>,
    browser_poll_timer: Res<BrowserPollingTimer>,
    //mut request_tiles_event: EventWriter<RequestTileUpdates>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
    mut browser_state: ResMut<NextState<BrowserIndexedDBStorageState>>,
    // mut game_time: ResMut<UpdateGameTimetamp>,
    // mut checkpoint_time: ResMut<CheckpointTimetamp>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
) {
    if browser_poll_timer.timer.just_finished() {
        info!("did this happen4");
        info!("indexedDBcheck");
        let res = comms_channel.rx.try_recv();

        match res {
            Ok(o) => {
                browser_state.set(BrowserIndexedDBStorageState::Off);
                info!("data made it!");
                info!("{:#?}", o);
                //let r_result = JsValue::from_str(&o);
                let a = serde_json::from_str::<TrimExplorerTileVec>(&o);
                match a {
                    Ok(o) => {
                        // info!("works!! {:#?}", o)

                        let mut holder_array = Vec::new();
                        for t in o.0 {
                            let resource = get_resource_for_tile(&t.x);
                            let land_index = get_land_index(t.h as u32, &resource, None);

                            let tt = TileData {
                                ln_address: "a@b.com".to_string(),
                                username: "bob".to_string(),
                                color: Srgba::hex("9000B0").unwrap().into(),
                                message: "".to_string(),
                                value: 32,
                                cost: 64,
                                height: t.h as u32,
                                land_index,
                                event_date: Utc::now(),
                                resource,
                                block_hash: DEFAULT_HASH.to_owned(),
                                block_time: 0,
                                block_bits: 0,
                                block_n_tx: 1,
                                block_size: 1,
                                block_fee: 1,
                                block_weight: 1,
                                block_ver: 1,
                            };
                            holder_array.push(tt.clone());
                            tile_map.map.insert(t.h as u32, tt.clone());
                        }
                        let land_index_map = calculate_index_for_resourced_lands(&mut tile_map.map);
                        *tile_map = land_index_map;

                        update_tile_event.send(UpdateTileTextureEvent(holder_array));
                    }
                    Err(e) => {
                        info!("error for serde_json::from {:#?}", e);
                    }
                }

                // match r_result {
                //     Ok(o) => {
                //         info!("we parsed it!!!");
                //         info!("{:#?}", o);
                //         // let world_map_converted = o.convert_trim_to_tilemap();

                //         // *tile_map = world_map_converted.clone();

                //         // request_tiles_event.send(RequestTileUpdates(RequestTileType::Ts));

                //         // let tiles = world_map_converted.to_tiledata_vec();

                //         // update_tile_event.send(UpdateTileTextureEvent(tiles));
                //     }
                //     Err(e) => {
                //         browser_state.set(BrowserIndexedDBStorageState::Off);
                //         info!("some error with parsing indexed DB browser storage {}", e);
                //     }
                // }
            }
            Err(e) => {
                info!("{:#?}", e);
            }
        }
    }
}
