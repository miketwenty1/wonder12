use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigText, NewBlockColorText, NewBlockLnAddressText, NewBlockMessageText,
    },
    keyboard::resources::KeyboardData,
    resourcey::{ColorPalette, ConfigAllCartBlocks, CurrentCartBlock, TargetType},
};

#[allow(clippy::type_complexity)]
pub fn write_keyboard_target(
    mut text_query: ParamSet<(
        Query<(&mut Text, &mut TextColor), With<NewBlockLnAddressText>>,
        Query<(&mut Text, &mut TextColor), With<NewBlockColorText>>,
        Query<(&mut Text, &mut TextColor), With<NewBlockMessageText>>,
        Query<(&mut Text, &mut TextColor), With<AllCartConfigText>>,
    )>,
    mut cart_config: ResMut<ConfigAllCartBlocks>,
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
                for (mut text, mut text_color) in text_query.p0().iter_mut() {
                    **text = keyboard.value.to_string();
                    **text_color = colors.text_color;

                    block_new_data.ln_address = keyboard.value.to_string();
                }
            }
            TargetType::NewColor => {
                for (mut text, mut text_color) in text_query.p1().iter_mut() {
                    **text = keyboard.value.to_string();
                    **text_color = colors.text_color;

                    clear_configbox = true;
                    block_new_data.color_text = keyboard.value.to_string();
                }
            }
            TargetType::NewMessage => {
                for (mut text, mut text_color) in text_query.p2().iter_mut() {
                    **text = keyboard.value.to_string();
                    **text_color = colors.text_color;

                    clear_configbox = true;
                    block_new_data.message = keyboard.value.to_string();
                }
            }
            _ => {}
        }
        if clear_configbox {
            for (mut text, _text_color) in text_query.p3().iter_mut() {
                **text = "".to_string();
                cart_config.0 = false;
            }
        }
    }
}
