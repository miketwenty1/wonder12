use self::{
    buy_details_menu::BuyDetailsMenuPlugin, go_to::GoToPlugin, qr_invoice::InvoiceQrUiPlugin,
    toast::ToastUiPlugin,
};
use bevy::prelude::*;

pub mod buy_details_menu;
pub mod go_to;
pub mod qr_invoice;
pub mod toast;

pub struct OverlayUiPlugin;

#[derive(Component, Debug)]
pub struct UiOverlay;

impl Plugin for OverlayUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuyDetailsMenuPlugin)
            .add_plugins(InvoiceQrUiPlugin)
            .add_plugins(ToastUiPlugin)
            .add_plugins(GoToPlugin);
    }
}
