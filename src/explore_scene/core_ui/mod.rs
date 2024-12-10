use bevy::prelude::*;
use event::{HideGameToggleChildren, HideMagnifyToggleChildren};
use inventory::InventoryMenuPlugin;
use paint_palette::PaintPalettePlugin;
use ui_right::{toggle_game_children, toggle_magnify_children};

use crate::statey::ExploreSceneState;

use self::{
    overall_ui::ui_explorer, ui_bottom::bottom_ui, ui_left::left_ui, ui_right::right_ui,
    ui_top::top_ui,
};

pub mod components;
pub mod event;
pub mod inventory;
pub mod overall_ui;
pub mod paint_palette;
pub mod ui_bottom;
pub mod ui_left;
pub mod ui_middle;
pub mod ui_right;
pub mod ui_top;

pub struct ExploreUiPlugin;

impl Plugin for ExploreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ExploreSceneState::On),
            (((
                ui_explorer,
                top_ui,
                left_ui,
                right_ui,
                bottom_ui,
                apply_deferred,
            )
                .chain())
            .run_if(run_once),),
        )
        .add_systems(Update, (toggle_game_children, toggle_magnify_children))
        .add_event::<HideGameToggleChildren>()
        .add_event::<HideMagnifyToggleChildren>()
        .add_plugins(InventoryMenuPlugin)
        .add_plugins(PaintPalettePlugin);
    }
}
