use bevy::prelude::*;

use crate::{resourcey::ColorPalette, statey::ToastState};

use super::{layout::spawn_toast, ToastEvent, ToastTimer, ToastType};

pub fn toast_event_reader(
    mut commands: Commands,
    mut reader: EventReader<ToastEvent>,
    mut state: ResMut<NextState<ToastState>>,
    colors: Res<ColorPalette>,
    asset_server: Res<AssetServer>,
) {
    for event in reader.read() {
        match event.ttype {
            ToastType::Good => {
                let bg_color = colors.accent_color;
                spawn_toast(
                    &mut commands,
                    &colors,
                    &asset_server,
                    bg_color,
                    event.message.to_string(),
                );
                state.set(ToastState::On);
            }
            ToastType::Bad => {
                let bg_color = colors.red_color;
                spawn_toast(
                    &mut commands,
                    &colors,
                    &asset_server,
                    bg_color,
                    event.message.to_string(),
                );
                state.set(ToastState::On);
            }
        };
    }
}

pub fn tick_api_receive_timer(mut timer: ResMut<ToastTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn despawn_toast_setter(timer: Res<ToastTimer>, mut state: ResMut<NextState<ToastState>>) {
    if timer.timer.just_finished() {
        state.set(ToastState::Off);
    }
}
