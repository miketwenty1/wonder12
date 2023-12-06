pub mod explore;
use bevy::prelude::*;

use crate::{despawn_screen, ExploreState};

use self::explore::{
    animate_sprites, clear_selection_button, detail_selection_button, edge_system,
    mouse_camera_system, select_tile, setup_explorer, spawn_block_sprites, touch_event_system,
    update_tile_textures, zoom_in_button_system, zoom_out_button_system,
};

pub struct ExplorePlugin;

#[derive(Component, Debug)]
pub struct UiOverlay;

impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(
                OnEnter(ExploreState::On),
                (setup_explorer).run_if(run_once()),
            )
            //run_once()
            .add_systems(
                Update,
                (
                    zoom_out_button_system,
                    zoom_in_button_system,
                    mouse_camera_system,
                    touch_event_system,
                    edge_system,
                    update_tile_textures,
                    spawn_block_sprites,
                    animate_sprites,
                    select_tile,
                    clear_selection_button,
                    detail_selection_button,
                )
                    .run_if(in_state(ExploreState::On)),
            );
        //.add_systems(OnExit(ExploreState::On), despawn_screen::<UiOverlay>);
    }
}
