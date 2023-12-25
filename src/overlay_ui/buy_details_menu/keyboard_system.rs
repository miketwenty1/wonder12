use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigText, NewBlockColorText, NewBlockLnAddressText, NewBlockMessageText,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    keyboard::resources::KeyboardData,
    resourcey::{ColorPalette, CurrentCartBlock, KeyboardTarget, TargetType},
};

#[allow(clippy::type_complexity)]
pub fn write_keyboard_target(
    mut text_query: ParamSet<(
        Query<&mut Text, With<NewBlockLnAddressText>>,
        Query<&mut Text, With<NewBlockColorText>>,
        Query<&mut Text, With<NewBlockMessageText>>,
        Query<&mut Text, With<AllCartConfigText>>,
    )>,
    keyboard_text: Res<KeyboardData>,
    target: ResMut<KeyboardTarget>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    colors: Res<ColorPalette>,
    //user: Res<User>,
) {
    if keyboard_text.is_changed() {
        let mut clear_configbox = false;
        match target.0 {
            TargetType::Nothing => {
                info!("this shouldn't ever be reached NOTHING targeted");
            }
            TargetType::NewLnAddress => {
                for mut text in text_query.p0().iter_mut() {
                    if *"" == keyboard_text.0 {
                        text.sections[0].value = DEFAULT_NEW_LN_TEXT.to_owned();
                        text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                    } else {
                        text.sections[0].value = keyboard_text.0.to_string();
                        text.sections[0].style.color = colors.text_color;
                    }
                    block_new_data.ln_address = keyboard_text.0.to_string();
                }
            }
            TargetType::NewColor => {
                for mut text in text_query.p1().iter_mut() {
                    if *"" == keyboard_text.0 {
                        text.sections[0].value = DEFAULT_NEW_COLOR_TEXT.to_owned();
                        text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                        //info!("why me? {}", keyboard_text.0);
                    } else {
                        //info!("why though? {}", keyboard_text.0);
                        text.sections[0].value = keyboard_text.0.to_string();
                        text.sections[0].style.color = colors.text_color;
                    }
                    clear_configbox = true;
                    block_new_data.color = keyboard_text.0.to_string();
                }
            }
            TargetType::NewMessage => {
                for mut text in text_query.p2().iter_mut() {
                    if *"" == keyboard_text.0 {
                        text.sections[0].value = DEFAULT_NEW_MESSAGE_TEXT.to_owned();
                        text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                    } else {
                        text.sections[0].value = keyboard_text.0.to_string();
                        text.sections[0].style.color = colors.text_color;
                    }
                    clear_configbox = true;
                    block_new_data.message = keyboard_text.0.to_string();
                }
            }
        }
        if clear_configbox {
            for mut text in text_query.p3().iter_mut() {
                text.sections[0].value = "".to_string();
            }
        }
    }
}
