use crate::{despawn_screen, statey::DrawState};
use bevy::prelude::*;

use self::{cleanup_draw::cleanup_system, draw_systems::update_system, layout_draw::spawn_layout};

pub mod cleanup_draw;
pub mod draw_keyboard_system;
pub mod draw_systems;
pub mod layout_draw;
pub struct DrawPlugin;

#[derive(Component, Debug)]
pub struct DrawScene;

impl Plugin for DrawPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_systems(OnEnter(DrawState::On), spawn_layout)
            .add_systems(Update, (update_system).run_if(in_state(DrawState::On)))
            .add_systems(
                OnExit(DrawState::On),
                (despawn_screen::<DrawScene>, cleanup_system),
            );
    }
}
