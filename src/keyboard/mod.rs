use bevy::prelude::*;

use self::{
    cleanup::cleanup_keyboard_system,
    event::ToggleKeyboardEvent,
    layout::setup_keyboard,
    resources::{
        tick_delete_init_timer_physical, tick_delete_init_timer_virtual,
        tick_delete_ongoing_timer_physical, tick_delete_ongoing_timer_vitual, AltTextToggle,
        DeleteTimerInitP, DeleteTimerInitV, DeleteTimerOnGoingP, DeleteTimerOnGoingV,
    },
    systems::{
        delete_physical_key_system, delete_virtual_key_system, physical_keyboard_system,
        virtual_capitalize_system, virtual_keyboard_system,
    },
};

mod cleanup;
pub mod components;
mod event;
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
            .insert_resource(AltTextToggle(false))
            .init_resource::<DeleteTimerInitV>()
            .init_resource::<DeleteTimerOnGoingV>()
            .init_resource::<DeleteTimerInitP>()
            .init_resource::<DeleteTimerOnGoingP>()
            .add_event::<ToggleKeyboardEvent>()
            //.add_systems(OnEnter(KeyboardState::On), setup_keyboard)
            .add_systems(
                Update,
                (
                    physical_keyboard_system,
                    virtual_keyboard_system,
                    virtual_capitalize_system,
                    tick_delete_init_timer_virtual,
                    tick_delete_init_timer_virtual,
                    tick_delete_init_timer_physical,
                    tick_delete_ongoing_timer_vitual,
                    tick_delete_ongoing_timer_physical,
                    delete_virtual_key_system,
                    delete_physical_key_system,
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
