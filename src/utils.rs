use bevy::{log::info, math::Vec2, render::color::Color};
use rand::Rng;
use regex::Regex;

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

pub fn logout_user(from_where: &str) {
    info!("USER LOGOUT! {}", from_where);
    let event_init = web_sys::CustomEventInit::new();
    let event = web_sys::CustomEvent::new_with_event_init_dict("logout", &event_init).unwrap();
    web_sys::window().unwrap().dispatch_event(&event).unwrap();
}

pub fn extract_number(input: &str) -> Option<i32> {
    let re = Regex::new(r"\d+").unwrap(); // matches one or more digits

    re.find(input)
        .and_then(|match_| match_.as_str().parse::<i32>().ok())
}
