use bevy::prelude::*;
pub const CRON_POLLING_TIME: f32 = 30.0;

#[derive(Resource)]
pub struct CronPollingTimer {
    pub timer: Timer,
}

impl Default for CronPollingTimer {
    fn default() -> CronPollingTimer {
        CronPollingTimer {
            timer: Timer::from_seconds(CRON_POLLING_TIME, TimerMode::Repeating),
        }
    }
}
