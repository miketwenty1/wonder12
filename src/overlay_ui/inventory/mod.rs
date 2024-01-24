use bevy::prelude::*;

use self::{
    event::AddInventoryRow,
    layout::spawn_layout,
    state::InventoryUiState,
    system::{inventory_adder_system, inventory_interaction},
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
            .add_systems(OnEnter(InventoryUiState::On), spawn_layout)
            .add_systems(
                Update,
                (inventory_interaction).run_if(in_state(InventoryUiState::On)),
            )
            .add_systems(Update, inventory_adder_system);
    }
}
