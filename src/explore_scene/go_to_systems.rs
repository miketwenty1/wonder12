use bevy::prelude::*;

use crate::{componenty::GoToBtn, overlay_ui::go_to::state::GoToUiState, resourcey::ColorPalette};

#[allow(clippy::type_complexity)]
pub fn go_to_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GoToBtn>),
    >,
    // mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
    mut ui_state: ResMut<NextState<GoToUiState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                ui_state.set(GoToUiState::On);
                info!("goto");
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.button_color.into();
            }
        }
    }
}
