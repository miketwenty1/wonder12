use bevy::prelude::*;

pub const API_POLLING_TIME: f32 = 1.0;

#[derive(Resource)]
pub struct BrowserPollingTimer {
    pub timer: Timer,
}

impl Default for BrowserPollingTimer {
    fn default() -> BrowserPollingTimer {
        BrowserPollingTimer {
            timer: Timer::from_seconds(API_POLLING_TIME, TimerMode::Repeating),
        }
    }
}

pub fn tick_browser_receive_timer(mut api_timer: ResMut<BrowserPollingTimer>, time: Res<Time>) {
    api_timer.timer.tick(time.delta());
}
