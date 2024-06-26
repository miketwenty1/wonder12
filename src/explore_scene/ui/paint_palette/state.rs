use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PaintPaletteUiState {
    On,
    #[default]
    Off,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MovementPaletteUiState {
    On,
    #[default]
    Off,
}
