use async_channel::{Receiver, Sender};
use bevy::ecs::system::Resource;

#[derive(Resource, Clone)]
pub struct TileDataChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct RequestInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct CheckInvoiceChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct UserBlockInventoryChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BrowserMapLocalStorageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BrowserCheckpointLocalStorageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BrowserIndexedDBStorageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct BlockMessagesStorageChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}
