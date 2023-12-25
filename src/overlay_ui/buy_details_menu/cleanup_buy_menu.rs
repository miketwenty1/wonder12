use bevy::prelude::*;

use crate::resourcey::CurrentCartBlock;

pub fn cleanup_keyboard_system(mut current_cart: ResMut<CurrentCartBlock>) {
    //current_cart.ln_address = "".to_string();
    current_cart.color = "".to_string();
    current_cart.message = "".to_string();
}
