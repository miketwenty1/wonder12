use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InventoryUiState {
    On,
    #[default]
    Off,
}
