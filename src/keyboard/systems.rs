use bevy::prelude::*;

use crate::resourcey::ColorPalette;

use super::{
    components::{Changeable, KeyBoardButton},
    event::ToggleKeyboardEvent,
    resources::{
        AltTextToggle, DeleteTimerInitP, DeleteTimerInitV, DeleteTimerOnGoingP,
        DeleteTimerOnGoingV, KeyboardData,
    },
};

// const ACCEPTABLE_CHARS: &str =
//     "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890";
const ACCEPTABLE_CHARS: &str =
    "1234567890=⌫!#$%*&'@()[]+-_,.:;?ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz⇧⬆\" ";
const MAX_INPUT_LENGTH: usize = 140;
#[allow(clippy::type_complexity)]
pub fn physical_keyboard_system(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<ButtonInput<KeyCode>>,
    mut keyboard_text: ResMut<KeyboardData>,
) {
    if keys.just_pressed(KeyCode::Backspace) {
        keyboard_text.value.pop();
    }

    for ev in char_evr.read() {
        let k = ev.char.to_string().chars().next().unwrap();

        if ACCEPTABLE_CHARS.contains(k) && keyboard_text.value.len() < MAX_INPUT_LENGTH {
            keyboard_text.value.push(k);
        } else {
            info!("no likey this character sorry")
        }

        //info!("new pkeydata {:?}", keyboard_text.0);
    }
}

#[allow(clippy::type_complexity)]
pub fn virtual_keyboard_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &KeyBoardButton),
        (Changed<Interaction>, With<Button>, With<KeyBoardButton>),
    >,
    mut keyboard_text: ResMut<KeyboardData>,
    mut c_toggle: ResMut<AltTextToggle>,
    colors: Res<ColorPalette>,
    mut event: EventWriter<ToggleKeyboardEvent>,
    // timer1
    // timer2:
) {
    for (interaction, mut color, keyboard_button) in &mut interaction_query {
        let k = keyboard_button.0;
        match *interaction {
            Interaction::Pressed => {
                match k {
                    '⌫' => {
                        keyboard_text.value.pop();
                    }
                    '⇧' => {
                        c_toggle.0 = !c_toggle.0;
                        debug!("capitalize/alt text is now set to: {}", c_toggle.0);
                        event.send(ToggleKeyboardEvent);
                    }
                    '⬆' => {
                        c_toggle.0 = !c_toggle.0;
                        debug!("capitalize/alt text is now set to: {}", c_toggle.0);
                        event.send(ToggleKeyboardEvent);
                    }
                    k if ACCEPTABLE_CHARS.contains(k)
                        && keyboard_text.value.len() < MAX_INPUT_LENGTH =>
                    {
                        if !c_toggle.0 {
                            keyboard_text.value.push(keyboard_button.0);
                        } else {
                            keyboard_text.value.push(keyboard_button.1);
                        }
                    }
                    _ => {
                        info!("no likey this character sorry")
                    }
                }

                info!("new vkeydata {:?}", keyboard_text.value);

                *color = colors.light_color.into();
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Ready?".to_string();
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = "Start".to_string();
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn delete_virtual_key_system(
    interaction_query: Query<(&Interaction, &KeyBoardButton)>,
    mut keyboard_text: ResMut<KeyboardData>,
    mut timer1: ResMut<DeleteTimerInitV>,
    timer2: Res<DeleteTimerOnGoingV>,
) {
    for (interaction, key) in interaction_query.iter() {
        if key.0 == '⌫' {
            if *interaction == Interaction::Pressed {
                timer1.init = true;
                if timer1.on && timer2.timer.just_finished() {
                    keyboard_text.value.pop();
                }
            } else {
                timer1.init = false;
                timer1.on = false;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn delete_physical_key_system(
    mut keyboard_text: ResMut<KeyboardData>,
    mut timer1: ResMut<DeleteTimerInitP>,
    keys: Res<ButtonInput<KeyCode>>,
    timer2: Res<DeleteTimerOnGoingP>,
) {
    if keys.pressed(KeyCode::Backspace) {
        timer1.init = true;
        if timer1.on && timer2.timer.just_finished() {
            keyboard_text.value.pop();
        }
    } else {
        timer1.init = false;
        timer1.on = false;
    }
}

pub fn virtual_capitalize_system(
    mut letter_query: Query<(&mut Text, &KeyBoardButton), With<Changeable>>,
    c_toggle: Res<AltTextToggle>,
    mut event: EventReader<ToggleKeyboardEvent>,
) {
    // if c_toggle.0 {
    //     for mut text in &mut letter_query {
    //         text.sections[0].value = text.sections[0].value.to_ascii_uppercase();
    //     }
    // } else {
    //     for mut text in &mut letter_query {
    //         text.sections[0].value = text.sections[0].value.to_ascii_lowercase();
    //     }
    // }
    for _e in event.read() {
        for (mut text, char_comp) in &mut letter_query {
            if !c_toggle.0 {
                //info!("first one");
                text.sections[0].value = char_comp.0.to_string();
            } else {
                //info!("second one");
                text.sections[0].value = char_comp.1.to_string();
            }
        }
    }
}
