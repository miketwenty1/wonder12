use crate::{despawn_screen, DisplayBuyUiState};
use bevy::prelude::*;

use self::{
    qr_code_layout::spawn_qr,
    systems::{cancel_qr_button_system, clean_up_qr, clipboard_button_system, expiration_text},
};

pub mod qr_code_layout;
pub mod systems;

pub struct InvoiceQrUiPlugin;

#[derive(Component, Debug)]
pub struct QrInvoiceOverlay;

impl Plugin for InvoiceQrUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(DisplayBuyUiState::Qr),
            (despawn_screen::<QrInvoiceOverlay>, clean_up_qr),
        )
        .add_systems(OnEnter(DisplayBuyUiState::Qr), spawn_qr)
        .add_systems(
            Update,
            (
                clipboard_button_system,
                cancel_qr_button_system,
                expiration_text,
            )
                .run_if(in_state(DisplayBuyUiState::Qr)),
        );
    }
}
