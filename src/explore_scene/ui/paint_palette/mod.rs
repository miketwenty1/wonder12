use bevy::prelude::*;
use draw::{draw_select_tile, mouse_draw_choose_tile, touch_draw_choose_tile};
use event::{ DrawSelectTileEvent, HideSelectedTiles, NewColorPicked, ViewSelectedTiles};
use layout::{hide_layout, highlight_pencil, show_layout, spawn_layout};
use resource::ViewablePaletteTiles;
use state::{PaintPaletteUiState, ToolPaletteUiState};
use system::{
    change_palette_selection, eraser_palette_button, eyedrop_palette_button, hide_selected_tiles, individual_color_palette_button, move_palette_button, new_color_picked_on_palette_event, pencil_palette_button, trash_palette_button, ui_interaction_enabled_buttons, ui_interaction_released_buttons, view_selected_tiles, viewhide_palette_button
};

use crate::statey::DisplayBuyUiState;

pub mod component;
pub mod layout;
pub mod resource;
pub mod state;
pub mod system;
pub struct PaintPalettePlugin;
pub mod draw;
pub mod event;

impl Plugin for PaintPalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PaintPaletteUiState::On),
            ((spawn_layout).run_if(run_once()),
            show_layout, highlight_pencil).chain()
        )
        .add_event::<NewColorPicked>()
        .add_event::<DrawSelectTileEvent>()
        .add_event::<ViewSelectedTiles>()
        .add_event::<HideSelectedTiles>()
        // .add_event::<ChangePaletteSelection>()
        .insert_resource(ViewablePaletteTiles(true))
        .add_systems
        (
            Update,
            (individual_color_palette_button, new_color_picked_on_palette_event, move_palette_button, 
                pencil_palette_button, trash_palette_button, eraser_palette_button, eyedrop_palette_button, 
                viewhide_palette_button, hide_selected_tiles, view_selected_tiles).run_if(in_state(PaintPaletteUiState::On)),
        )
        .add_systems
        (Update,
            (
                ui_interaction_enabled_buttons,

                ((mouse_draw_choose_tile, draw_select_tile, touch_draw_choose_tile)
                .run_if(not(in_state(ToolPaletteUiState::Off).or(in_state(ToolPaletteUiState::Move)))
                .and(in_state(PaintPaletteUiState::On)
                .and(in_state(DisplayBuyUiState::Off))))),

                ui_interaction_released_buttons

        ).chain())
        //.add_systems(Update, (change_palette_selection).run_if(state_changed::<ToolPaletteUiState>))
        // .add_systems(
        //     Update,
        //     .run_if(in_state(ToolPaletteUiState::Eraser))
        // )
        .add_systems(
            OnExit(PaintPaletteUiState::On),
            hide_layout
        )
        // .add_systems(
        //     Update,
        //     (
                // inventory_remover_system,
                // inventory_adder_system,
                // visible_inventory_toggle_button,
                // inventory_colorbox_buttons,
            // ),
        //)
        ;
    }
}
