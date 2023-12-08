pub mod amount_ui;
pub mod explore;
pub mod toggle_ui;
pub mod update_toggle_events;

use bevy::prelude::*;

use crate::ExploreState;

use self::{
    amount_ui::{setup_amount_selected_text, update_amount_selected_text},
    explore::{
        animate_sprites, clear_selection_button, detail_selection_button, edge_system,
        mouse_camera_system, select_tile, setup_explorer, spawn_block_sprites, touch_event_system,
        update_tile_textures, zoom_in_button_system, zoom_out_button_system,
    },
    toggle_ui::{
        setup_toggle, toggle_button_sub_system_toggle1, toggle_button_sub_system_toggle2,
        toggle_button_sub_system_toggle3, toggle_button_sub_system_toggle4, toggle_button_system,
    },
    update_toggle_events::{buildings_visibility_event, change_tile_text_event, land_color_event},
};

pub struct ExplorePlugin;

#[derive(Component, Debug)]
pub struct UiOverlay;

impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                OnEnter(ExploreState::On),
                (setup_explorer, setup_toggle, setup_amount_selected_text).run_if(run_once()),
            )
            //run_once()
            .add_systems(
                Update,
                (
                    (
                        zoom_out_button_system,
                        zoom_in_button_system,
                        clear_selection_button,
                        detail_selection_button,
                        touch_event_system,
                        toggle_button_system,
                        toggle_button_sub_system_toggle1,
                        toggle_button_sub_system_toggle2,
                        toggle_button_sub_system_toggle3,
                        toggle_button_sub_system_toggle4,
                        mouse_camera_system,
                    )
                        .chain(),
                    edge_system,
                    update_tile_textures,
                    spawn_block_sprites,
                    animate_sprites,
                    select_tile,
                    buildings_visibility_event,
                    land_color_event,
                    change_tile_text_event,
                    update_amount_selected_text,
                )
                    .run_if(in_state(ExploreState::On)),
            ); //.add_systems(OnExit(ExploreState::On), despawn_screen::<UiOverlay>);
    }
}
