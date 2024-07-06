use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PaintPaletteUiState {
    On,
    #[default]
    Off,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ToolPaletteUiState {
    Pencil,
    Move,
    Eraser,
    Eyedrop,
    ViewHide,
    #[default]
    Off,
}
