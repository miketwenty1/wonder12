use bevy::{log::info, math::Vec2, render::color::Color};
use rand::Rng;
use regex::Regex;

// fn convert_to_string(value: f32) -> String {
//     // Ensure the input value is within the range 0.0 to 1.0
//     let clamped_value = value.clamp(0.0, 1.0);

//     // Convert to range 0 to 255
//     let int_value = (clamped_value * 255.0).round() as u8;

//     // Format the integer with leading zeros to ensure a minimum length of 3 characters
//     format!("{:03}", int_value)
// }

// pub fn convert_color_to_hexstring(value: Color) -> String {
//     let r = convert_to_string(value.r());
//     let g = convert_to_string(value.g());
//     let b = convert_to_string(value.b());
//     let concat = format!("{}{}{}", r, g, b);
//     info!("concat {}, {}, {}", r, g, b);
//     let hex_color_string = get_color_hex(concat.as_str());
//     info!("hexer {}", hex_color_string);
//     hex_color_string
// }

// pub fn get_color_hex(decimal_string: &str) -> String {
//     if decimal_string.len() != 9 || !decimal_string.chars().all(|c| c.is_ascii_digit()) {
//         return "Invalid input. Please provide a string of 9 digits.".to_string();
//     }

//     // Split the string into three parts for R, G, and B
//     let (r, rest) = decimal_string.split_at(3);
//     let (g, b) = rest.split_at(3);

//     // Convert each part to a value between 0 and 255
//     let r_val = r.parse::<u32>().unwrap() * 255 / 999;
//     let g_val = g.parse::<u32>().unwrap() * 255 / 999;
//     let b_val = b.parse::<u32>().unwrap() * 255 / 999;

//     // Convert the values to hex and return the combined string
//     format!("#{:02X}{:02X}{:02X}", r_val, g_val, b_val)
// }

pub fn convert_color_to_hexstring(c: Color) -> String {
    // Ensure the input values are within the range [0, 1]
    let r = (c.r().clamp(0.0, 1.0) * 255.0).round() as u8;
    let g = (c.g().clamp(0.0, 1.0) * 255.0).round() as u8;
    let b = (c.b().clamp(0.0, 1.0) * 255.0).round() as u8;
    info!("color c: {:?}", c);
    info!("r-{}, g-{}, b-{}", r, g, b);
    // Format into a hexadecimal string
    format!("{:02X}{:02X}{:02X}", r, g, b)
}

pub fn is_valid_email_format_string(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,9})+$").unwrap();
    email_regex.is_match(email)
}

pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let g: f32 = rng.gen_range(0.0..1.0);
    let b: f32 = rng.gen_range(0.0..1.0);

    //info!("getting a random color: {}-{}-{}", r, g, b);
    Color::Rgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    }
}

pub fn distance_between_vecs(a: &Vec2, b: &Vec2) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    (dx.powi(2) + dy.powi(2)).sqrt()
}
