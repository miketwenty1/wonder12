use bevy::prelude::*;

use crate::{
    eventy::TravelHeight,
    keyboard::{resources::KeyboardData, KeyboardState},
    resourcey::{ColorPalette, MaxBlockHeight, TargetType},
    statey::ExploreState,
};

use super::{
    component::{GoToBackBtn, GoToGoBtn},
    state::GoToUiState,
};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GoToBackBtn>),
    >,
    //mut text_query: Query<&mut Text>,
    mut overlay_state: ResMut<NextState<GoToUiState>>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
    colors: Res<ColorPalette>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut keyboard: ResMut<KeyboardData>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                //text.sections[0].value = button_text;
                explore_state.set(ExploreState::On);
                keyboard_state.set(KeyboardState::Off);
                // help with jumpiness when leaving this screen - hopefully
                mouse.clear();
                keyboard.target = TargetType::Nothing;
                keyboard.value = "".to_string();
                overlay_state.set(GoToUiState::Off);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.red_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn go_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GoToGoBtn>),
    >,
    //mut text_query: Query<&mut Text>,
    mut overlay_state: ResMut<NextState<GoToUiState>>,
    //mut explore_state: ResMut<NextState<ExploreState>>,
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
    colors: Res<ColorPalette>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut keyboard: ResMut<KeyboardData>,
    max_block_height: Res<MaxBlockHeight>,
    mut travel: EventWriter<TravelHeight>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                //text.sections[0].value = button_text;
                // explore_state.set(ExploreState::On);
                keyboard_state.set(KeyboardState::Off);
                // help with jumpiness when leaving this screen - hopefully
                mouse.clear();
                keyboard.target = TargetType::Nothing;
                overlay_state.set(GoToUiState::Off);
                let block_height = keyboard.clone().value;
                match block_height.parse::<u32>() {
                    Ok(number) => {
                        let travel_height = if number > max_block_height.0 {
                            max_block_height.0
                        } else {
                            number
                        };
                        info!("travel height {}", travel_height);
                        travel.send(TravelHeight(travel_height));
                    }
                    Err(e) => info!("Did you input an invalid blockheight? {}", e),
                }
                keyboard.value = "".to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
    if keys.pressed(KeyCode::Enter) {
        //text.sections[0].value = button_text;
        // explore_state.set(ExploreState::On);
        keyboard_state.set(KeyboardState::Off);
        // help with jumpiness when leaving this screen - hopefully
        mouse.clear();
        keyboard.target = TargetType::Nothing;
        overlay_state.set(GoToUiState::Off);
        let block_height = keyboard.clone().value;
        match block_height.parse::<u32>() {
            Ok(number) => {
                let travel_height = if number > max_block_height.0 {
                    max_block_height.0
                } else {
                    number
                };
                info!("travel height {}", travel_height);
                travel.send(TravelHeight(travel_height));
            }
            Err(e) => info!("Did you input an invalid blockheight? {}", e),
        }
        keyboard.value = "".to_string();
    }
}
