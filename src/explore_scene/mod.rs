// pub mod amount_ui;
pub mod cron_systems;
pub mod explore;
// pub mod mouse_movement_systems;
// pub mod selection;
// pub mod toggle_ui;
// pub mod touch_movement_systems;
// pub mod update_after_purchase;
// pub mod update_toggle_events;
// pub mod zoom;

use bevy::prelude::*;

use crate::ExploreState;

use self::{
    cron_systems::CronPollingTimer,
    explore::{init_explorer, spawn_block_sprites},
};

pub struct ExplorePlugin;

impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CronPollingTimer>()
            .add_systems(
                OnEnter(ExploreState::On),
                ((init_explorer).run_if(run_once()),),
            )
            .add_systems(
                Update,
                (spawn_block_sprites,).run_if(in_state(ExploreState::On)),
            );
    }
}
