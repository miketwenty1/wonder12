use bevy::prelude::*;

use crate::{
    browser::event::WriteLocalBrowserStorage,
    componenty::{InitLoadingText, UiOverlayingExplorerButton},
    eventy::ClearLastSelectedTile,
    resourcey::{InitBlockCount, WorldOwnedTileMap},
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
    init: Res<InitBlockCount>,
    mut state: ResMut<NextState<InitLoadingBlocksState>>,
    mut browser: EventWriter<WriteLocalBrowserStorage>,
) {
    for mut text in &mut text_query {
        let blocks_loaded = tilemap.map.len();
        let percentage = (blocks_loaded as f32 / (init.0 as f32)) * 100.0;

        text.sections[0].value = format!("Initilizing Game Map {}%", percentage as u32);

        if percentage >= 100.0 {
            info!("yarr we initilized");
            state.set(InitLoadingBlocksState::Off);
            browser.send(WriteLocalBrowserStorage);
        }
    }
}
