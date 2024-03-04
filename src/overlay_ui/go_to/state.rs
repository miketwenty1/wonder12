use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GoToUiState {
    On,
    #[default]
    Off,
}
