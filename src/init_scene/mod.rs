use bevy::prelude::*;
use canvas::fit_canvas_to_parent;
use channels::init_js_comms_channels;
use init_res::init_hardcoded_res;
use spritesheet::setup_spritesheets;

use crate::{
    browser::BrowserPlugin,
    comms::CommsPlugin,
    explore_scene::{overlay_ui::OverlayUiPlugin, ExplorePlugin},
    keyboard::KeyboardPlugin,
    statey::InitSceneState,
};
pub mod canvas;
pub mod channels;
pub mod init_res;
pub mod spritesheet;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(InitSceneState::On),
            (
                fit_canvas_to_parent,
                init_hardcoded_res,
                setup_spritesheets,
                init_js_comms_channels,
            )
                .chain()
                .run_if(run_once),
        )
        .add_plugins((
            CommsPlugin,
            OverlayUiPlugin,
            ExplorePlugin,
            KeyboardPlugin,
            BrowserPlugin,
        ));
    }
}
