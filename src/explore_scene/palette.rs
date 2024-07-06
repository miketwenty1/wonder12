use bevy::prelude::*;

use crate::{
    componenty::{DrawBtn, DrawBtnImage},
    eventy::ClearManualSelectionEvent,
    explore_scene::core_ui::paint_palette::state::ToolPaletteUiState,
    resourcey::ColorPalette,
};

use super::core_ui::paint_palette::{event::ViewSelectedTiles, state::PaintPaletteUiState};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn draw_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<DrawBtn>),
    >,
    mut draw_image_q: Query<&mut UiImage, (With<DrawBtnImage>, Without<DrawBtn>)>,
    // mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
    mut ui_state: ResMut<NextState<PaintPaletteUiState>>,
    paint_palette_state: Res<State<PaintPaletteUiState>>,
    mut movement_palette_state: ResMut<NextState<ToolPaletteUiState>>,
    asset_server: Res<AssetServer>,
    mut clear_event: EventWriter<ClearManualSelectionEvent>,
    mut palette_tiles_view_event: EventWriter<ViewSelectedTiles>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                for mut image in &mut draw_image_q {
                    *color = UiImage::new(asset_server.load("ui/palette_120x120.png"));
                    if *paint_palette_state == PaintPaletteUiState::On {
                        palette_tiles_view_event.send(ViewSelectedTiles);
                        ui_state.set(PaintPaletteUiState::Off);
                        clear_event.send(ClearManualSelectionEvent);
                        info!("draw off");
                        *image = UiImage::new(asset_server.load("ui/blank_120x120.png"));
                    } else {
                        ui_state.set(PaintPaletteUiState::On);
                        movement_palette_state.set(ToolPaletteUiState::Pencil);
                        clear_event.send(ClearManualSelectionEvent);
                        info!("draw on");
                        *image = UiImage::new(asset_server.load("ui/cancel_120x120.png"));
                    }
                }
            }
            Interaction::Hovered => {
                *color = UiImage::new(asset_server.load("ui/palette_120x120.png"))
                    .with_color(colors.accent_color);
                //colors.accent_color.into();
            }
            Interaction::None => {
                *color = UiImage::new(asset_server.load("ui/palette_120x120.png"))
                    .with_color(colors.light_color);
            }
        }
    }
}
