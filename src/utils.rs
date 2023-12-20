use bevy::render::color::Color;
use rand::Rng;
use regex::Regex;

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
    //info!("concat {}, {}, {}", r, g, b);
    let hex_color_string = all_colors::get_color_hex(concat.as_str());

    hex_color_string
}

pub fn is_valid_email_format_string(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,3})+$").unwrap();
    email_regex.is_match(email)
}

pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let g: f32 = rng.gen_range(0.0..1.0);
    let b: f32 = rng.gen_range(0.0..1.0);
    let new_color = Color::Rgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    };
    new_color
}
