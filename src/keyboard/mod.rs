use bevy::prelude::*;

use self::{
    cleanup::cleanup_keyboard_system,
    layout::setup_keyboard,
    resources::CapitalizeToggle,
    systems::{physical_keyboard_system, virtual_capitalize_system, virtual_keyboard_system},
};

mod cleanup;
pub mod components;
pub mod layout;
pub mod resources;
mod systems;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum KeyboardState {
    #[default]
    Off,
    On,
}

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .insert_resource(CapitalizeToggle(false))
            //.add_systems(OnEnter(KeyboardState::On), setup_keyboard)
            .add_systems(
                Update,
                (
                    physical_keyboard_system,
                    virtual_keyboard_system,
                    virtual_capitalize_system,
                )
                    .run_if(in_state(KeyboardState::On)),
            )
            .add_systems(Update, setup_keyboard)
            .add_systems(
                OnExit(KeyboardState::On),
                cleanup_keyboard_system, //(despawn_screen::<KeyBoard>, cleanup_keyboard_system),
            );
    }
}
