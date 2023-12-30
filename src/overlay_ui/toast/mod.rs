use bevy::prelude::*;

use crate::{despawn_screen, statey::ToastState};

use self::systems::{despawn_toast_setter, tick_api_receive_timer, toast_event_reader};

pub const TOAST_TIME: f32 = 4.0;

pub mod layout;
pub mod systems;
pub struct ToastUiPlugin;

#[derive(Resource)]
pub struct ToastTimer {
    pub timer: Timer,
}

impl Default for ToastTimer {
    fn default() -> ToastTimer {
        ToastTimer {
            timer: Timer::from_seconds(TOAST_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event, Debug)]
pub struct ToastEvent {
    pub ttype: ToastType,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum ToastType {
    Good,
    Bad,
}

#[derive(Component)]
pub struct ToastNode;

#[derive(Component)]
pub struct ToastInnerNode;

#[derive(Component)]
pub struct ToastText;

impl Plugin for ToastUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastTimer>()
            .add_event::<ToastEvent>()
            .add_systems(
                Update,
                (tick_api_receive_timer, despawn_toast_setter).run_if(in_state(ToastState::On)),
            )
            .add_systems(Update, toast_event_reader)
            .add_systems(OnExit(ToastState::On), despawn_screen::<ToastNode>);
    }
}
//AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
