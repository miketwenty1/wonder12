pub mod ui;
use bevy::prelude::*;

use crate::{despawn_screen, DisplayUiState};

use self::ui::{back_button_system, go_button_system, spawn_layout};

pub struct OverlayUiPlugin;

#[derive(Component, Debug)]
pub struct UiOverlay;

impl Plugin for OverlayUiPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(DisplayUiState::On), spawn_layout)
            .add_systems(
                Update,
                (go_button_system, back_button_system).run_if(in_state(DisplayUiState::On)),
            )
            .add_systems(OnExit(DisplayUiState::On), despawn_screen::<UiOverlay>);
    }
}
