use crate::resourcey::ColorPalette;

use super::{
    component::{PaletteBtn, PaletteMoveBtn},
    resource::MovementPaletteSelected,
    state::MovementPaletteUiState,
};
use bevy::prelude::*;

#[allow(clippy::type_complexity)]
pub fn general_palette_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (With<PaletteBtn>, Without<PaletteMoveBtn>),
        ),
    >,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.green_color.into();
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.light_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn move_palette_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PaletteMoveBtn>),
    >,
    colors: Res<ColorPalette>,
    mut selected: ResMut<MovementPaletteSelected>,
    mut movement_palette_state: ResMut<NextState<MovementPaletteUiState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *selected = MovementPaletteSelected(!selected.0);

                if selected.0 {
                    movement_palette_state.set(MovementPaletteUiState::Off);
                } else {
                    movement_palette_state.set(MovementPaletteUiState::On);
                }

                *color = colors.green_color.into();
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                if selected.0 {
                    *color = colors.accent_color.into();
                } else {
                    *color = colors.light_color.into();
                }
            }
        }
    }
}
