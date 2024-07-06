use bevy::prelude::*;

use crate::{despawn_screen, keyboard::resources::KeyboardData};

use self::{
    component::GoToNode,
    keyboard_system::goto_write_keyboard_target,
    layout::spawn_layout,
    state::GoToUiState,
    system::{back_button_system, go_button},
};

pub mod component;
pub mod event;
pub mod keyboard_system;
pub mod layout;
pub mod state;
pub mod system;

pub struct GoToPlugin;

impl Plugin for GoToPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GoToUiState::On), spawn_layout)
            .add_systems(
                Update,
                (
                    back_button_system,
                    go_button,
                    (goto_write_keyboard_target).run_if(resource_changed::<KeyboardData>),
                ),
            )
            .add_systems(OnExit(GoToUiState::On), despawn_screen::<GoToNode>);
    }
}
