use bevy::prelude::*;

use crate::keyboard::styles::PRESSED_BUTTON;

use super::{
    components::{Capitalizable, KeyBoardButton},
    resources::{CapitalizeToggle, KeyboardData},
    styles::{HOVERED_BUTTON, NORMAL_BUTTON},
};

const ACCEPTABLE_CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 @.";
const MAX_INPUT_LENGTH: usize = 100;
#[allow(clippy::type_complexity)]
pub fn physical_keyboard_system(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut keyboard_text: ResMut<KeyboardData>,
) {
    if keys.just_pressed(KeyCode::Back) {
        keyboard_text.0.pop();
    }

    for ev in char_evr.read() {
        let k = ev.char;

        if ACCEPTABLE_CHARS.contains(k) && keyboard_text.0.len() < MAX_INPUT_LENGTH {
            keyboard_text.0.push(k);
        } else {
            info!("no likey this character sorry")
        }

        info!("new pkeydata {:?}", keyboard_text.0);
    }
}

#[allow(clippy::type_complexity)]
pub fn virtual_keyboard_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &KeyBoardButton),
        (Changed<Interaction>, With<Button>, With<KeyBoardButton>),
    >,
    mut keyboard_text: ResMut<KeyboardData>,
    mut c_toggle: ResMut<CapitalizeToggle>,
) {
    for (interaction, mut color, keyboard_button) in &mut interaction_query {
        let k = keyboard_button.0;
        match *interaction {
            Interaction::Pressed => {
                match k {
                    '<' => {
                        keyboard_text.0.pop();
                    }
                    '^' => {
                        c_toggle.0 = !c_toggle.0;
                        debug!("capitalize is now set to: {}", c_toggle.0);
                    }
                    k if ACCEPTABLE_CHARS.contains(k)
                        && keyboard_text.0.len() < MAX_INPUT_LENGTH =>
                    {
                        if c_toggle.0 {
                            keyboard_text.0.push(k.to_ascii_uppercase());
                        } else {
                            keyboard_text.0.push(k);
                        }
                    }
                    _ => {
                        info!("no likey this character sorry")
                    }
                }

                info!("new vkeydata {:?}", keyboard_text.0);

                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Ready?".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Start".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn virtual_capitalize_system(
    mut letter_query: Query<&mut Text, With<Capitalizable>>,
    c_toggle: Res<CapitalizeToggle>,
) {
    if c_toggle.0 {
        for mut text in &mut letter_query {
            text.sections[0].value = text.sections[0].value.to_ascii_uppercase();
        }
    } else {
        for mut text in &mut letter_query {
            text.sections[0].value = text.sections[0].value.to_ascii_lowercase();
        }
    }
}
