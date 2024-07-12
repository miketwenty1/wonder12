use bevy::{color::palettes::css::WHITE, prelude::*};

use crate::{
    componenty::GoToBtn, explore_scene::overlay_ui::go_to::state::GoToUiState,
    resourcey::ColorPalette,
};

#[allow(clippy::type_complexity)]
pub fn go_to_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GoToBtn>),
    >,
    // mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
    mut ui_state_c: ResMut<NextState<GoToUiState>>,
    ui_state: Res<State<GoToUiState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = WHITE.into();
                if **ui_state == GoToUiState::Off {
                    ui_state_c.set(GoToUiState::On);
                    info!("goto");
                } else {
                    ui_state_c.set(GoToUiState::Off);
                    info!("goto off");
                }
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
