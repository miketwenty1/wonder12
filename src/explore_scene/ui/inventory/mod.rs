use bevy::prelude::*;
use system::{inventory_colorbox_buttons, visible_inventory_toggle_button};

use self::{
    event::AddInventoryRow,
    layout::spawn_layout,
    state::InventoryUiState,
    system::{inventory_adder_system, inventory_remover_system},
};

pub mod component;
pub mod layout;
pub mod state;
pub mod system;
pub struct InventoryMenuPlugin;
pub mod event;

impl Plugin for InventoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddInventoryRow>()
            .add_systems(
                OnEnter(InventoryUiState::On),
                (spawn_layout).run_if(run_once()),
            )
            .add_systems(
                Update,
                (
                    inventory_remover_system,
                    inventory_adder_system,
                    visible_inventory_toggle_button,
                    inventory_colorbox_buttons,
                ),
            );
    }
}
