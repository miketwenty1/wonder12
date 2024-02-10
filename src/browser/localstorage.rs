use crate::resourcey::{BrowserCheckpointLocalStorageChannel, BrowserMapLocalStorageChannel};
use crate::resourcey::{UpdateGameTimetamp, WorldOwnedTileMap};
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_futures::{js_sys, wasm_bindgen};

use super::event::{ReadLocalBrowserStorage, WriteLocalBrowserStorage};
use super::resource::BrowserPollingTimer;

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
                    info!("good checkpoint {:#?}", o);
                    let _ = checkpoint_cc.try_send(o.as_string().unwrap_or_default());
                }
                Err(e) => {
                    info!("error from checkpoint local browser storage {:#?}", e);
                    let _ = checkpoint_cc.try_send("errorornotfound".to_string());
                }
            }
            match mapdata_result {
                Ok(o) => {
                    info!("good mapdata {:#?}", o);
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

pub fn readcheck_local_storage(
    map_channel: Res<BrowserMapLocalStorageChannel>,
    checkpoint_channel: Res<BrowserCheckpointLocalStorageChannel>,
    browser_poll_timer: Res<BrowserPollingTimer>,
) {
    if browser_poll_timer.timer.just_finished() {
        info!("ticky boy");
        let map_res = map_channel.rx.try_recv();
        let checkpoint_res = checkpoint_channel.rx.try_recv();

        info!(
            "checkpoint_res: {:#?}, map_res: {:#?}",
            checkpoint_res, map_res
        );
    }
}
