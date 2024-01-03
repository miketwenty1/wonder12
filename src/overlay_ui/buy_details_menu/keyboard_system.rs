use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigText, NewBlockColorText, NewBlockLnAddressText, NewBlockMessageText,
    },
    keyboard::resources::KeyboardData,
    resourcey::{ColorPalette, CurrentCartBlock, TargetType},
};

#[allow(clippy::type_complexity)]
pub fn write_keyboard_target(
    mut text_query: ParamSet<(
        Query<&mut Text, With<NewBlockLnAddressText>>,
        Query<&mut Text, With<NewBlockColorText>>,
        Query<&mut Text, With<NewBlockMessageText>>,
        Query<&mut Text, With<AllCartConfigText>>,
    )>,
    keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    colors: Res<ColorPalette>,
    //user: Res<User>,
) {
    if keyboard.is_changed() {
        let mut clear_configbox = false;
        match keyboard.target {
            TargetType::Nothing => {}
            TargetType::NewLnAddress => {
                for mut text in text_query.p0().iter_mut() {
                    text.sections[0].value = keyboard.value.to_string();
                    text.sections[0].style.color = colors.text_color;

                    block_new_data.ln_address = keyboard.value.to_string();
                }
            }
            TargetType::NewColor => {
                for mut text in text_query.p1().iter_mut() {
                    text.sections[0].value = keyboard.value.to_string();
                    text.sections[0].style.color = colors.text_color;

                    clear_configbox = true;
                    block_new_data.color_text = keyboard.value.to_string();
                }
            }
            TargetType::NewMessage => {
                for mut text in text_query.p2().iter_mut() {
                    text.sections[0].value = keyboard.value.to_string();
                    text.sections[0].style.color = colors.text_color;

                    clear_configbox = true;
                    block_new_data.message = keyboard.value.to_string();
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
