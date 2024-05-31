use bevy::prelude::*;

use crate::{componenty::DrawBtn, resourcey::ColorPalette, statey::DrawState};

#[allow(clippy::type_complexity)]
pub fn draw_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<DrawBtn>),
    >,
    // mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
    mut ui_state: ResMut<NextState<DrawState>>,
    state: Res<State<DrawState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();

                if *state == DrawState::On {
                    ui_state.set(DrawState::Off);
                    info!("draw off");
                } else {
                    ui_state.set(DrawState::On);
                    info!("draw on");
                }
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
