use bevy::prelude::*;

use crate::{
    browser::event::WriteBrowserStorage,
    componenty::{InitLoadingText, UiOverlayingExplorerButton},
    eventy::ClearLastSelectedTile,
    resourcey::{BlockExplorerCount, LocalBrowserStorageCount, WorldOwnedTileMap},
    statey::InitLoadingBlocksState,
};

pub fn clear_last_selected_tile_ui_button(
    mut clear_last_selected_tile_event: EventWriter<ClearLastSelectedTile>,
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<UiOverlayingExplorerButton>),
    >,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                clear_last_selected_tile_event.send(ClearLastSelectedTile);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

pub fn init_block_loading_text(
    mut text_query: Query<&mut Text, With<InitLoadingText>>,
    tilemap: Res<WorldOwnedTileMap>,
    local_browaer_storage_count: Res<LocalBrowserStorageCount>,
    indexeddb_count: Res<BlockExplorerCount>,
    //mut state: ResMut<NextState<InitLoadingBlocksState>>,
    cur_state: Res<State<InitLoadingBlocksState>>,
    mut browser: EventWriter<WriteBrowserStorage>,
) {
    for mut text in &mut text_query {
        let (load_type, total_count) = match **cur_state {
            InitLoadingBlocksState::Off => ("All Done", 0),
            InitLoadingBlocksState::LocalBrowserStorage => {
                ("Game Tiles", local_browaer_storage_count.0)
            }
            InitLoadingBlocksState::IndexedDB => ("Blockchain Tiles", indexeddb_count.0),
        };

        let blocks_loaded = tilemap.map.len();
        let percentage = (blocks_loaded as f32 / (total_count as f32)) * 100.0;

        **text = format!("Initilizing {} {}%", load_type, percentage as u32);

        if percentage >= 100.0 {
            info!("yarr we initilized");
            //state.set(InitLoadingBlocksState::Off);
            browser.send(WriteBrowserStorage);
        }
    }
}
