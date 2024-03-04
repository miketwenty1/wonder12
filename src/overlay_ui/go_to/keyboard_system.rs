use bevy::prelude::*;

use crate::{
    keyboard::resources::KeyboardData,
    resourcey::{ColorPalette, CurrentCartBlock, TargetType},
};

use super::component::GoToTextBoxText;

#[allow(clippy::type_complexity, clippy::single_match)]
pub fn goto_write_keyboard_target(
    mut text_query: ParamSet<(Query<&mut Text, With<GoToTextBoxText>>,)>,
    keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    colors: Res<ColorPalette>,
    //user: Res<User>,
) {
    if keyboard.is_changed() {
        match keyboard.target {
            TargetType::GoTo => {
                for mut text in text_query.p0().iter_mut() {
                    text.sections[0].value = keyboard.value.to_string();
                    text.sections[0].style.color = colors.text_color;

                    block_new_data.ln_address = keyboard.value.to_string();
                }
            }
            _ => {}
        }
    }
}
