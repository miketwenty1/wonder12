use bevy::{color::Srgba, log::info, math::Vec2};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use regex::Regex;

use crate::consty::MINIMUM_BLOCK_AMOUNT;

pub fn convert_color_to_hexstring(c: Srgba) -> String {
    // // Ensure the input values are within the range [0, 1]
    // let r = (c.red.clamp(0.0, 1.0) * 255.0).round() as u8;
    // let g = (c.green.clamp(0.0, 1.0) * 255.0).round() as u8;
    // let b = (c.blue.clamp(0.0, 1.0) * 255.0).round() as u8;
    // //info!("color c: {:?}", c);
    // //info!("r-{}, g-{}, b-{}", r, g, b);
    // // Format into a hexadecimal string
    // format!("{:02X}{:02X}{:02X}", r, g, b)
    let s = c.to_hex();
    // removes #
    s.chars().filter(|&c| c != '#').collect()
}

pub fn is_valid_email_format_string(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^\w+([\.-]?\w+)*@\w+([\.-]?\w+)*(\.\w{2,9})+$").unwrap();
    email_regex.is_match(email)
}

pub fn get_random_color() -> Srgba {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..1.0);
    let g: f32 = rng.gen_range(0.0..1.0);
    let b: f32 = rng.gen_range(0.0..1.0);

    //info!("getting a random color: {}-{}-{}", r, g, b);
    Srgba {
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

pub fn derive_cost_from_value(v: u32) -> u32 {
    if v == 0 {
        MINIMUM_BLOCK_AMOUNT
    } else {
        v * 2
    }
}

// pub fn get_ts_with_micro() -> DateTime<Utc> {
//     // let format = "%Y-%m-%d %H:%M:%S.%6f %Z";
//     // let datetime_utc = NaiveDateTime::parse_from_str(&o, format);
//     let now = Utc::now();
//     let micros_nanos = (now.nanosecond() / 1_000) * 1_000;
//     let ts = now.with_nanosecond(micros_nanos).expect("Invalid DateTime");
//     info!("returning {}", ts);
//     ts
// }
pub fn to_millisecond_precision(dt: DateTime<Utc>) -> DateTime<Utc> {
    // Get the total number of milliseconds in the current second
    let milliseconds = dt.timestamp_subsec_millis();

    // Calculate the difference in microseconds to subtract
    let micros_to_subtract = dt.timestamp_subsec_micros() - (milliseconds * 1_000);

    // Subtract the extra microseconds to align to milliseconds

    dt - Duration::microseconds(micros_to_subtract as i64)
}
