use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CommsApiState {
    #[default]
    Off,
    //SetName,
    //LoadBlockData,
    ReceiveInvoice,
    //Buy,
    CheckInvoice,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CommsApiBlockLoadState {
    #[default]
    Off,
    LoadBlockData,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CommsApiInventoryState {
    #[default]
    Off,
    On,
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
    BlockDetail,
    Qr,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum InitLoadingBlocksState {
    Off,
    #[default]
    On,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum ToastState {
    #[default]
    Off,
    On,
}
