use crate::{
    despawn_screen, keyboard::resources::KeyboardData, statey::ExploreState, DisplayBuyUiState,
};
use bevy::prelude::*;

use self::{layout::spawn_layout, state::InventoryUiState, system::inventory_interaction};

pub mod component;
pub mod layout;
pub mod state;
pub mod system;
pub struct InventoryMenuPlugin;

impl Plugin for InventoryMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InventoryUiState::On), spawn_layout)
            .add_systems(
                Update,
                (inventory_interaction).run_if(in_state(InventoryUiState::On)),
            );
    }
}
