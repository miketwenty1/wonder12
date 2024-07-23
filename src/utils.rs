use bevy::{
    color::Srgba,
    log::info,
    math::Vec2,
    prelude::IntoSystem,
    utils::{hashbrown, HashMap},
};
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use regex::Regex;
use ulam::Quad;

use crate::{
    consty::MINIMUM_BLOCK_AMOUNT,
    resourcey::{TileData, WorldOwnedTileMap},
    structy::TileResource,
};

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

pub fn get_resource_for_tile(block_hash: &str) -> TileResource {
    // Ensure the block_hash is at least 2 characters long
    if block_hash.len() < 2 {
        return TileResource::Unknown;
    }

    // Get the last two characters
    let last_two_chars = &block_hash[block_hash.len() - 2..];

    // Convert the last two characters to a number
    let last_two_num = u8::from_str_radix(last_two_chars, 16).unwrap_or(255);
    info!("last 2 nums of hash {:?}", last_two_num);
    // Match the number to the corresponding TileResource using ranges
    match last_two_num {
        0..=2 => TileResource::Mountain,
        3..=5 => TileResource::Water,
        6..=210 => TileResource::Grass,
        211..=252 => TileResource::Forest,
        253..=255 => TileResource::Desert,
        // _ => TileResource::Unknown, // Handle any unexpected characters
    }
}

pub fn get_land_index(
    height: u32,
    resource: &TileResource,
    tile_map: Option<&hashbrown::HashMap<u32, TileData>>,
) -> usize {
    match tile_map {
        Some(s) => {
            info!("defaulting to 22 for get_land_index");
            22
        }
        None => resource.spritesheet_index_value(),
    }
}

pub fn crunch_index_number(r1: &TileResource, r2: &TileResource, r3: &TileResource) -> usize {
    if *r1 != TileResource::Grass {
        return r1.spritesheet_index_value();
    }

    let mut values = vec![
        r1.spritesheet_index_value(),
        r2.spritesheet_index_value(),
        r3.spritesheet_index_value(),
    ];
    values.sort();
    match values.as_slice() {
        [0, 0, 0] => 0,
        [0, 0, 1] => 6,
        [0, 0, 2] => 12,
        [0, 0, 3] => 18,
        [0, 0, 4] => 24,
        [0, 1, 1] => 7,
        [0, 1, 2] => 5,
        [0, 1, 3] => 17,
        [0, 1, 4] => 11,
        [0, 2, 2] => 8,
        [0, 2, 3] => 29,
        [0, 2, 4] => 23,
        [0, 3, 3] => 9,
        [0, 3, 4] => 30,
        [0, 4, 4] => 10,
        [1, 1, 1] => 1,
        [1, 1, 2] => 13,
        [1, 1, 3] => 19,
        [1, 1, 4] => 25,
        [1, 2, 2] => 14,
        [1, 2, 3] => 32,
        [1, 2, 4] => 31,
        [1, 3, 3] => 15,
        [1, 3, 4] => 33,
        [1, 4, 4] => 16,
        [2, 2, 2] => 2,
        [2, 2, 3] => 20,
        [2, 2, 4] => 26,
        [2, 3, 3] => 21,
        [2, 3, 4] => 34,
        [2, 4, 4] => 22,
        [3, 3, 3] => 3,
        [3, 3, 4] => 27,
        [3, 4, 4] => 28,
        [4, 4, 4] => 4,
        _ => 35,
    }
}

pub fn calculate_index_for_resourced_lands(
    tile_map: &mut HashMap<u32, TileData>,
) -> WorldOwnedTileMap {
    let mut new_tile_map: HashMap<u32, TileData> = HashMap::new();
    let tile_map2 = tile_map.clone();

    for (height, tile) in tile_map.iter_mut() {
        let current = tile.resource.clone();
        let quad = ulam::quad_of_value(*height);
        let inside = match quad {
            ulam::Quad::North => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x, c.y - 1);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::NorthEast => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x - 1, c.y);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::East => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x - 1, c.y);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::SouthEast => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x - 1, c.y);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::South => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x, c.y + 1);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::SouthWest => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x + 1, c.y);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::West => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x + 1, c.y);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            ulam::Quad::Center => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x, c.y - 1);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
            _ => {
                let c = ulam::calc_coord::calc_coord(*height);
                let insideheight = ulam::value_of_xy(c.x + 1, c.y);
                let inside_resource = tile_map2.get(&insideheight);
                match inside_resource {
                    Some(s) => &s.resource,
                    None => &current,
                }
            }
        };

        let previous = if quad == Quad::Center {
            &current
        } else {
            let a = tile_map2.get(&(height - 1));
            match a {
                Some(s) => &s.resource,
                None => &current,
            }
        };
        // we want the resource of the previous

        // match on QUAD type for this res
        // and we want the resource of the inside unless a corner or right corner + 1 or center.. then just give the same resource from height.

        let new_index = crunch_index_number(&current, previous, inside);

        tile.set_index(new_index);
        new_tile_map.insert(*height, tile.clone());
    }
    WorldOwnedTileMap { map: new_tile_map }
}
