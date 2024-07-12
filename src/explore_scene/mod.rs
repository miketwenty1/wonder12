pub mod amount_ui;
pub mod core_ui;
pub mod cron_systems;
pub mod desktop_movement_systems;
pub mod explore;
pub mod explorer_overlay_system;
pub mod go_to_systems;
pub mod overlay_ui;
pub mod palette;
pub mod selection;
pub mod touch_movement_systems;
pub mod travel;
pub mod update_after_purchase;
pub mod update_toggle_events;
pub mod zoom;

use bevy::prelude::*;
use core_ui::paint_palette::state::{PaintPaletteUiState, ToolPaletteUiState};
use explore::clear_manual_selection;
use palette::draw_button_system;
use zoom::zoom_wheel_system;

use crate::{
    componenty::InitLoadingNode,
    despawn_screen,
    statey::{ExploreSelectState, InitLoadingBlocksState},
    ExploreState,
};

use self::{
    amount_ui::update_amount_selected_text,
    core_ui::{
        ui_right::{
            toggle_button_sub_system_toggle1, toggle_button_sub_system_toggle2,
            toggle_button_sub_system_toggle3, toggle_button_sub_system_toggle4,
            toggle_button_system,
        },
        ExploreUiPlugin,
    },
    cron_systems::{cron_update_tiles, tick_update_tile_cron_timer, CronPollingTimer},
    desktop_movement_systems::{
        clear_last_selected_tile, keyboard_movement_camera_system, mouse_movement_camera_system,
    },
    explore::{
        animate_sprites, buy_selection_button, clear_selection, clear_selection_button,
        edge_system, init_explorer, spawn_block_sprites, update_tile_textures,
    },
    explorer_overlay_system::{clear_last_selected_tile_ui_button, init_block_loading_text},
    go_to_systems::go_to_button_system,
    selection::{choose_tile, select_tile},
    touch_movement_systems::touch_event_system,
    travel::travel_event,
    update_after_purchase::update_tiles_after_purchase,
    update_toggle_events::{buildings_visibility_event, change_tile_text_event, land_color_event},
    zoom::{
        cam_ortho_scale_text_visibility, pinch_system, zoom_in_button_system,
        zoom_out_button_system,
    },
};

pub struct ExplorePlugin;

impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ExploreUiPlugin)
            .init_resource::<CronPollingTimer>()
            // OnEnter State Systems
            .add_systems(
                OnEnter(ExploreState::On),
                (init_explorer).run_if(run_once()),
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
                        toggle_button_system,
                        toggle_button_sub_system_toggle1,
                        toggle_button_sub_system_toggle2,
                        toggle_button_sub_system_toggle3,
                        toggle_button_sub_system_toggle4,
                        (
                            choose_tile,
                            mouse_movement_camera_system,
                            touch_event_system,
                            pinch_system,
                        )
                            .run_if(
                                in_state(ToolPaletteUiState::Off)
                                    .or(in_state(ToolPaletteUiState::Move)),
                            ),
                    )
                        .chain(),
                    zoom_wheel_system,
                    keyboard_movement_camera_system,
                    edge_system,
                    spawn_block_sprites,
                    buildings_visibility_event,
                    land_color_event,
                    change_tile_text_event,
                    clear_selection,
                    clear_manual_selection,
                    clear_last_selected_tile,
                    clear_last_selected_tile_ui_button,
                    draw_button_system,
                    go_to_button_system,
                )
                    .run_if(in_state(ExploreState::On)),
            )
            .add_systems(
                Update,
                (
                    (select_tile.run_if(in_state(PaintPaletteUiState::Off))),
                    apply_deferred,
                    update_amount_selected_text,
                )
                    .chain()
                    .run_if(in_state(ExploreSelectState::On)),
            )
            .add_systems(
                Update,
                (
                    update_tile_textures,
                    animate_sprites,
                    cron_update_tiles.run_if(in_state(InitLoadingBlocksState::Off)),
                    tick_update_tile_cron_timer,
                    update_tiles_after_purchase,
                    cam_ortho_scale_text_visibility,
                    travel_event,
                ),
            );
        //.add_systems(OnExit(ExploreState::On), reset_mouse);
    }
}
