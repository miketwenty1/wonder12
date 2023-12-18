use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CommsApiState {
    #[default]
    Off,
    SetName,
    LoadBlockData,
    ReceiveInvoice,
    Buy,
    CheckInvoice,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ExploreState {
    On,
    #[default]
    Off,
    Paused,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum DisplayBuyUiState {
    #[default]
    Off,
    On,
    Qr,
}
