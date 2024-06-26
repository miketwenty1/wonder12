use std::ptr::null;

use bevy::prelude::*;

use crate::{
    componenty::{DrawBtn, DrawBtnImage},
    eventy::ClearSelectionEvent,
    explore_scene::ui::paint_palette::state::MovementPaletteUiState,
    resourcey::ColorPalette,
};

use super::ui::paint_palette::state::PaintPaletteUiState;

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn draw_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DrawBtn>),
    >,
    mut draw_image_q: Query<&mut UiImage, (With<DrawBtnImage>, Without<DrawBtn>)>,
    // mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
    mut ui_state: ResMut<NextState<PaintPaletteUiState>>,
    paint_palette_state: Res<State<PaintPaletteUiState>>,
    mut movement_palette_state: ResMut<NextState<MovementPaletteUiState>>,
    asset_server: Res<AssetServer>,
    mut clear_event: EventWriter<ClearSelectionEvent>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for mut image in &mut draw_image_q {
                    *color = colors.light_color.into();

                    if *paint_palette_state == PaintPaletteUiState::On {
                        ui_state.set(PaintPaletteUiState::Off);
                        info!("draw off");
                        *image = UiImage::new(asset_server.load("ui/blank_120x120.png"));
                    } else {
                        ui_state.set(PaintPaletteUiState::On);
                        movement_palette_state.set(MovementPaletteUiState::On);
                        clear_event.send(ClearSelectionEvent);
                        info!("draw on");
                        *image = UiImage::new(asset_server.load("ui/cancel_120x120.png"));
                    }
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = BackgroundColor(Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                })
            }
        }
    }
}
