use self::{
    buy_details_menu::BuyDetailsMenuPlugin, go_to::GoToPlugin, qr_invoice::InvoiceQrUiPlugin,
    toast::ToastUiPlugin,
};
use bevy::prelude::*;

pub mod buy_details_menu;
pub mod go_to;
pub mod qr_invoice;
pub mod toast;

/// Overlays don't use the core ui grid
pub struct OverlayUiPlugin;

impl Plugin for OverlayUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BuyDetailsMenuPlugin)
            .add_plugins(InvoiceQrUiPlugin)
            .add_plugins(ToastUiPlugin)
            .add_plugins(GoToPlugin);
    }
}
