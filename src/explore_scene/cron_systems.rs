use bevy::prelude::*;

use crate::{eventy::RequestTileUpdates, structy::RequestTileType};
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

pub fn tick_update_tile_cron_timer(mut timer: ResMut<CronPollingTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

#[allow(clippy::too_many_arguments)]
pub fn cron_update_tiles(timer: Res<CronPollingTimer>, mut event: EventWriter<RequestTileUpdates>) {
    if timer.timer.finished() {
        info!("ping");
        event.send(RequestTileUpdates(RequestTileType::Ts));
    }
}
