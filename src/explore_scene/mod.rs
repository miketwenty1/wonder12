pub mod amount_ui;
pub mod cron_systems;
pub mod explore;
pub mod explorer_movement_systems;
pub mod explorer_overlay_system;
pub mod toggle_ui;
pub mod update_toggle_events;

use bevy::prelude::*;

use crate::{
    componenty::InitLoadingNode, despawn_screen, statey::InitLoadingBlocksState, ExploreState,
};

use self::{
    amount_ui::{setup_amount_selected_text, update_amount_selected_text},
    cron_systems::{cron_update_tiles, tick_update_tile_cron_timer, CronPollingTimer},
    explore::{
        animate_sprites, buy_selection_button, clear_selection, clear_selection_button,
        desktop_movement_camera_system, edge_system, init_explorer, select_tile,
        spawn_block_sprites, touch_event_system, update_tile_textures, zoom_in_button_system,
        zoom_out_button_system,
    },
    explorer_movement_systems::clear_last_selected_tile,
    explorer_overlay_system::{clear_last_selected_tile_ui_button, init_block_loading_text},
    toggle_ui::{
        setup_toggle, toggle_button_sub_system_toggle1, toggle_button_sub_system_toggle2,
        toggle_button_sub_system_toggle3, toggle_button_sub_system_toggle4, toggle_button_system,
    },
    update_toggle_events::{buildings_visibility_event, change_tile_text_event, land_color_event},
};

pub struct ExplorePlugin;

impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CronPollingTimer>()
            // OnEnter State Systems
            .add_systems(
                OnEnter(ExploreState::On),
                (
                    (init_explorer, setup_toggle, setup_amount_selected_text).run_if(run_once()),
                    // setup_explorer,
                ),
            )
            .add_systems(
                Update,
                (init_block_loading_text).run_if(in_state(InitLoadingBlocksState::On)),
            )
            .add_systems(
                OnExit(InitLoadingBlocksState::On),
                despawn_screen::<InitLoadingNode>,
            )
            .add_systems(
                Update,
                (
                    (
                        zoom_out_button_system,
                        zoom_in_button_system,
                        clear_selection_button,
                        buy_selection_button,
                        touch_event_system,
                        toggle_button_system,
                        toggle_button_sub_system_toggle1,
                        toggle_button_sub_system_toggle2,
                        toggle_button_sub_system_toggle3,
                        toggle_button_sub_system_toggle4,
                        desktop_movement_camera_system,
                    )
                        .chain(),
                    edge_system,
                    spawn_block_sprites,
                    (select_tile, apply_deferred, update_amount_selected_text).chain(),
                    buildings_visibility_event,
                    land_color_event,
                    change_tile_text_event,
                    clear_selection,
                    clear_last_selected_tile,
                    clear_last_selected_tile_ui_button,
                )
                    .run_if(in_state(ExploreState::On)),
            )
            .add_systems(
                Update,
                (
                    update_tile_textures,
                    animate_sprites,
                    cron_update_tiles,
                    tick_update_tile_cron_timer,
                ),
            ); //.add_systems(OnExit(ExploreState::On), despawn_screen::<UiOverlay>);
    }
}
