use crate::async_resource_comm_channels::{
    BrowserCheckpointLocalStorageChannel, BrowserMapLocalStorageChannel,
};
use crate::comms::structy::TrimTileLocalBrowserStorage;
use crate::eventy::{RequestTileUpdates, UpdateTileTextureEvent};
use crate::resourcey::CheckpointTimetamp;
use crate::resourcey::{UpdateGameTimetamp, WorldOwnedTileMap};
use crate::structy::RequestTileType;
use bevy::prelude::*;

use chrono::{NaiveDateTime, Timelike};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::{js_sys, wasm_bindgen};

use super::event::{ReadLocalBrowserStorage, WriteLocalBrowserStorage};
use super::resource::BrowserPollingTimer;
use super::state::BrowserStorageState;

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
    fn retrieveCheckpoint() -> js_sys::Promise;
}

pub fn request_local_storage(
    mut event: EventReader<ReadLocalBrowserStorage>,
    map_channel: Res<BrowserMapLocalStorageChannel>,
    checkpoint_channel: Res<BrowserCheckpointLocalStorageChannel>,
) {
    for _e in event.read() {
        let map_cc = map_channel.tx.clone();
        let checkpoint_cc = checkpoint_channel.tx.clone();
        spawn_local(async move {
            let mapdata_promise = retrieveLocalBrowserGameData();
            let checkpoint_promise = retrieveCheckpoint();
            let checkpoint_result = JsFuture::from(checkpoint_promise).await;
            let mapdata_result = JsFuture::from(mapdata_promise).await;

            match checkpoint_result {
                Ok(o) => {
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
                    // info!("good mapdata {:#?}", o);
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

#[allow(clippy::too_many_arguments)]
pub fn readcheck_local_storage(
    map_channel: Res<BrowserMapLocalStorageChannel>,
    checkpoint_channel: Res<BrowserCheckpointLocalStorageChannel>,
    browser_poll_timer: Res<BrowserPollingTimer>,
    mut request_tiles_event: EventWriter<RequestTileUpdates>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
    mut browser_state: ResMut<NextState<BrowserStorageState>>,
    mut game_time: ResMut<UpdateGameTimetamp>,
    mut checkpoint_time: ResMut<CheckpointTimetamp>,
    mut update_tile_event: EventWriter<UpdateTileTextureEvent>,
) {
    if browser_poll_timer.timer.just_finished() {
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
                                            browser_state.set(BrowserStorageState::Off);
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
                info!("e: {}", e);
            }
        }
    }
}
