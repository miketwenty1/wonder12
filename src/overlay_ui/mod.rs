pub mod buyui;
use bevy::prelude::*;

use crate::{despawn_screen, DisplayBuyUiState};

use self::buyui::{
    back_button_system, buy_button_system, left_cart_button_system, right_cart_button_system,
    spawn_layout,
};

pub struct OverlayUiPlugin;

#[derive(Component, Debug)]
pub struct UiOverlay;

impl Plugin for OverlayUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(DisplayBuyUiState::On), spawn_layout)
            .add_systems(
                Update,
                (
                    buy_button_system,
                    back_button_system,
                    right_cart_button_system,
                    left_cart_button_system,
                )
                    .run_if(in_state(DisplayBuyUiState::On)),
            )
            .add_systems(OnExit(DisplayBuyUiState::On), despawn_screen::<UiOverlay>);
    }
}
