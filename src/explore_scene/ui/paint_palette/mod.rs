use bevy::prelude::*;
use layout::{hide_layout, show_layout, spawn_layout};
use resource::MovementPaletteSelected;
use state::PaintPaletteUiState;
use system::{general_palette_buttons, move_palette_buttons};

pub mod component;
pub mod layout;
pub mod resource;
pub mod state;
pub mod system;
pub struct PaintPalettePlugin;
pub mod event;

impl Plugin for PaintPalettePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(PaintPaletteUiState::On),
            ((spawn_layout).run_if(run_once()),
            show_layout).chain()
        )
        .insert_resource(MovementPaletteSelected(false))
        .add_systems(
            Update,
            (general_palette_buttons, move_palette_buttons).run_if(in_state(PaintPaletteUiState::On)),
        )
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
