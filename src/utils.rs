use bevy::prelude::*;
use bevy::render::color::Color;

fn convert_to_string(value: f32) -> String {
    // Ensure the input value is within the range 0.0 to 1.0
    let clamped_value = value.clamp(0.0, 1.0);

    // Convert to range 0 to 255
    let int_value = (clamped_value * 255.0).round() as u8;

    // Format the integer with leading zeros to ensure a minimum length of 3 characters
    format!("{:03}", int_value)
}

pub fn convert_color_to_hexstring(value: Color) -> String {
    let r = convert_to_string(value.r());
    let g = convert_to_string(value.g());
    let b = convert_to_string(value.b());
    let concat = format!("{}{}{}", r, g, b);
    info!("concat {}, {}, {}", r, g, b);
    let hex_color_string = all_colors::get_color_hex(concat.as_str());

    hex_color_string
}
