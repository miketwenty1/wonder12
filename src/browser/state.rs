use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BrowserLocalStorageState {
    Off,
    #[default]
    On,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum BrowserIndexedDBStorageState {
    #[default]
    Off,
    On,
}
