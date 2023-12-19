use self::{
    cleanup_buy_menu::cleanup_keyboard_system,
    keyboard_system::write_keyboard_target,
    layout_buy_menu::{set_keyboard, spawn_layout},
    qr_code_layout::{spawn_qr, UiQrOverlay},
    systems_buyui::{
        back_button_system, buy_button_system, config_cart_button_system,
        leftright_cart_button_system, leftright_cart_button_system_set_new_text,
        new_color_button_system, new_ln_address_button_system, new_message_button_system,
        set_default_text_for_empty_text,
    },
};
use crate::{despawn_screen, keyboard::resources::KeyboardData, DisplayBuyUiState};
use bevy::prelude::*;

pub mod cleanup_buy_menu;
pub mod keyboard_system;
pub mod layout_buy_menu;
pub mod qr_code_layout;
pub mod systems_buyui;
pub struct OverlayUiPlugin;

#[derive(Component, Debug)]
pub struct UiOverlay;

impl Plugin for OverlayUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                OnEnter(DisplayBuyUiState::BlockDetail),
                (spawn_layout, set_keyboard).chain(),
            )
            .add_systems(
                Update,
                (
                    buy_button_system,
                    back_button_system,
                    (
                        leftright_cart_button_system,
                        leftright_cart_button_system_set_new_text,
                    )
                        .chain(),
                    config_cart_button_system,
                    new_ln_address_button_system,
                    new_color_button_system,
                    new_message_button_system,
                    set_default_text_for_empty_text,
                    (write_keyboard_target).run_if(resource_changed::<KeyboardData>()),
                )
                    .run_if(in_state(DisplayBuyUiState::BlockDetail)),
            )
            // .add_systems(
            //     Update,
            //         .and_then(not(resource_added::<KeyboardData>())),
            // )
            .add_systems(
                OnExit(DisplayBuyUiState::BlockDetail),
                (despawn_screen::<UiOverlay>, cleanup_keyboard_system),
            )
            .add_systems(
                OnExit(DisplayBuyUiState::Qr),
                (despawn_screen::<UiQrOverlay>, cleanup_keyboard_system),
            )
            .add_systems(OnEnter(DisplayBuyUiState::Qr), spawn_qr);
    }
}
