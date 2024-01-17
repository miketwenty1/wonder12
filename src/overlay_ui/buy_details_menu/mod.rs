use self::{
    cleanup_buy_menu::cleanup_keyboard_system,
    keyboard_system::write_keyboard_target,
    layout_buy_menu::spawn_layout,
    systems_buyui::{
        back_button_system, buy_button_system, config_cart_button_system, highlight_box_system,
        leftright_cart_button_system, leftright_cart_button_system_set_new_text,
        new_color_button_system, new_ln_address_button_system, new_message_button_system,
        set_default_text_for_empty_text, show_color_button_system, tab_key_system,
    },
};
use crate::{despawn_screen, keyboard::resources::KeyboardData, DisplayBuyUiState};
use bevy::prelude::*;

pub mod cleanup_buy_menu;
pub mod keyboard_system;
pub mod layout_buy_menu;
pub mod systems_buyui;

pub struct BuyDetailsMenuPlugin;

#[derive(Component, Debug)]
pub struct BuyDetailsMenu;

impl Plugin for BuyDetailsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                OnEnter(DisplayBuyUiState::BlockDetail),
                (spawn_layout, apply_deferred).chain(),
            )
            .add_systems(
                Update,
                (
                    buy_button_system,
                    back_button_system,
                    (
                        leftright_cart_button_system,
                        leftright_cart_button_system_set_new_text,
                        new_message_button_system,
                        new_color_button_system,
                        new_ln_address_button_system,
                        set_default_text_for_empty_text,
                    )
                        .chain(),
                    tab_key_system,
                    highlight_box_system,
                    show_color_button_system,
                    (write_keyboard_target).run_if(resource_changed::<KeyboardData>),
                    config_cart_button_system,
                    //set_keyboard,
                )
                    .run_if(in_state(DisplayBuyUiState::BlockDetail)),
            )
            .add_systems(
                OnExit(DisplayBuyUiState::BlockDetail),
                (despawn_screen::<BuyDetailsMenu>, cleanup_keyboard_system),
            );
        // .add_systems(
        //     OnExit(DisplayBuyUiState::Qr),
        //     (despawn_screen::<UiQrOverlay>, cleanup_keyboard_system),
        // )
        //.add_systems(OnEnter(DisplayBuyUiState::Qr), spawn_qr)
        // .add_systems(
        //     Update,
        //     (clipboard_button_system).run_if(in_state(DisplayBuyUiState::Qr)),
        // );
    }
}
