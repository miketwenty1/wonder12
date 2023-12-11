use bevy::prelude::*;

use super::resources::KeyboardData;

pub fn cleanup_keyboard_system(mut keyboard_text: ResMut<KeyboardData>) {
    keyboard_text.0 = "".to_string();
}
