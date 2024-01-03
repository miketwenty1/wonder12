use bevy::prelude::*;

use crate::resourcey::TargetType;

use super::resources::KeyboardData;

pub fn cleanup_keyboard_system(mut keyboard_text: ResMut<KeyboardData>) {
    keyboard_text.value = "".to_string();
    keyboard_text.target = TargetType::Nothing;
}
