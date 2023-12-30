use bevy::render::color::Color;

pub const CHUNK_PIXEL_SIZE: f32 = 400.0;
pub const TILE_SCALE: f32 = 3.0;
pub const TILE_PIXEL_SIZE: f32 = 32.0;
//const TILE_PADDING_SIZE: f32 = 0.0;
pub const TOTAL_TILE_SCALE_SIZE: f32 = TILE_PIXEL_SIZE * TILE_SCALE + 4.0;
pub const CHUNK_TILE_SPAN_COUNT: i32 = (CHUNK_PIXEL_SIZE / TOTAL_TILE_SCALE_SIZE) as i32;
pub const DESPAWN_TILE_THRESHOLD: i32 = 51 + CHUNK_TILE_SPAN_COUNT * 4;
pub const CAMERA_SANITY_FACTOR: f32 = 1.25;
pub const MOVE_VELOCITY_FACTOR: f32 = 20.0;

pub const MINIMUM_BLOCK_AMOUNT: u32 = 128;

pub const DEFAULT_NEW_LN_TEXT: &str = "Type in a lightning address!";
pub const DEFAULT_NEW_COLOR_TEXT: &str = "Random Color, if not specified.";
pub const DEFAULT_NEW_MESSAGE_TEXT: &str = "Leave an optional message for others to see";

pub const DEFAULT_NO_PICK_COLOR: Color = Color::Rgba {
    red: 0.7,
    green: 0.7,
    blue: 0.7,
    alpha: 1.0,
};

pub const ZOOM_IN_MAX: f32 = 0.25;
pub const ZOOM_OUT_MAX: f32 = 5.0;

pub const DARKEST_BUILDING: Color = Color::Rgba {
    red: 0.15,
    green: 0.15,
    blue: 0.15,
    alpha: 1.0,
};

pub const MAX_SELECTION_SIZE: usize = 100;
