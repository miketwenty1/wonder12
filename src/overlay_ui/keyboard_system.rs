use bevy::prelude::*;

use crate::{
    componenty::{NewBlockColorText, NewBlockLnAddressText, NewBlockMessageText},
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR, PICKED_COLOR,
    },
    keyboard::resources::KeyboardData,
    resourcey::{CurrentCartBlock, KeyboardTarget, TargetType},
};

#[allow(clippy::type_complexity)]
pub fn write_keyboard_target(
    mut text_query: ParamSet<(
        Query<(&mut Text, &NewBlockLnAddressText), With<NewBlockLnAddressText>>,
        Query<(&mut Text, &NewBlockColorText), With<NewBlockColorText>>,
        Query<(&mut Text, &NewBlockMessageText), With<NewBlockMessageText>>,
    )>,
    keyboard_text: Res<KeyboardData>,
    target: ResMut<KeyboardTarget>,
    mut block_new_data: ResMut<CurrentCartBlock>,
) {
    match target.0 {
        TargetType::Nothing => {
            info!("this shouldn't ever be reached NOTHING targeted");
        }
        TargetType::NewLnAddress => {
            for (mut text, default_text) in text_query.p0().iter_mut() {
                if *"" == keyboard_text.0 {
                    text.sections[0].value = DEFAULT_NEW_LN_TEXT.to_owned();
                    text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                } else {
                    text.sections[0].value = keyboard_text.0.to_string();
                    text.sections[0].style.color = PICKED_COLOR;
                }
                block_new_data.ln_address = keyboard_text.0.to_string();
            }
        }
        TargetType::NewColor => {
            for (mut text, default_text) in text_query.p1().iter_mut() {
                if *"" == keyboard_text.0 {
                    text.sections[0].value = DEFAULT_NEW_COLOR_TEXT.to_owned();
                    text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                } else {
                    text.sections[0].value = keyboard_text.0.to_string();
                    text.sections[0].style.color = PICKED_COLOR;
                }
                block_new_data.color = keyboard_text.0.to_string();
            }
        }
        TargetType::NewMessage => {
            for (mut text, default_text) in text_query.p2().iter_mut() {
                if *"" == keyboard_text.0 {
                    text.sections[0].value = DEFAULT_NEW_MESSAGE_TEXT.to_owned();
                    text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                } else {
                    text.sections[0].value = keyboard_text.0.to_string();
                    text.sections[0].style.color = PICKED_COLOR;
                }
                block_new_data.message = keyboard_text.0.to_string();
            }
        }
    }
}
