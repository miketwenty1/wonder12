use bevy::prelude::*;

use crate::resourcey::TargetType;

#[derive(Resource)]
pub struct AltTextToggle(pub bool);

#[derive(Resource, Clone)]
pub struct KeyboardData {
    pub value: String,
    pub target: TargetType,
}

pub const DELETE_TIMER_INIT: f32 = 1.0;

#[derive(Resource)]
pub struct DeleteTimerInitV {
    pub timer: Timer,
    pub init: bool,
    pub on: bool,
}

impl Default for DeleteTimerInitV {
    fn default() -> DeleteTimerInitV {
        DeleteTimerInitV {
            timer: Timer::from_seconds(DELETE_TIMER_INIT, TimerMode::Repeating),
            init: false,
            on: false,
        }
    }
}

#[derive(Resource)]
pub struct DeleteTimerInitP {
    pub timer: Timer,
    pub init: bool,
    pub on: bool,
}

impl Default for DeleteTimerInitP {
    fn default() -> DeleteTimerInitP {
        DeleteTimerInitP {
            timer: Timer::from_seconds(DELETE_TIMER_INIT, TimerMode::Repeating),
            init: false,
            on: false,
        }
    }
}
pub fn tick_delete_init_timer_virtual(mut timer: ResMut<DeleteTimerInitV>, time: Res<Time>) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() && timer.init {
        timer.on = true;
    }
}

pub fn tick_delete_init_timer_physical(mut timer: ResMut<DeleteTimerInitP>, time: Res<Time>) {
    timer.timer.tick(time.delta());

    if timer.timer.just_finished() && timer.init {
        timer.on = true;
    }
}

pub const DELETE_TIMER_ONGOING: f32 = 0.06;

#[derive(Resource)]
pub struct DeleteTimerOnGoingV {
    pub timer: Timer,
}

impl Default for DeleteTimerOnGoingV {
    fn default() -> DeleteTimerOnGoingV {
        DeleteTimerOnGoingV {
            timer: Timer::from_seconds(DELETE_TIMER_ONGOING, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct DeleteTimerOnGoingP {
    pub timer: Timer,
}

impl Default for DeleteTimerOnGoingP {
    fn default() -> DeleteTimerOnGoingP {
        DeleteTimerOnGoingP {
            timer: Timer::from_seconds(DELETE_TIMER_ONGOING, TimerMode::Repeating),
        }
    }
}

pub fn tick_delete_ongoing_timer_vitual(mut timer: ResMut<DeleteTimerOnGoingV>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn tick_delete_ongoing_timer_physical(mut timer: ResMut<DeleteTimerOnGoingP>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}
