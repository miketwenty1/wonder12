use bevy::color::Srgba;

pub const CHUNK_PIXEL_SIZE: f32 = 400.0;
pub const TILE_SCALE: f32 = 3.0;
pub const TILE_PIXEL_SIZE: f32 = 32.0;
//const TILE_PADDING_SIZE: f32 = 0.0;
pub const TOTAL_TILE_SCALE_SIZE: f32 = TILE_PIXEL_SIZE * TILE_SCALE + 4.0;
pub const CHUNK_TILE_SPAN_COUNT: i32 = (CHUNK_PIXEL_SIZE / TOTAL_TILE_SCALE_SIZE) as i32;
pub const CHUNK_TILE_SPAN_MULTIPLIER: i32 = 8;
pub const DESPAWN_TILE_THRESHOLD: i32 = 51 + CHUNK_TILE_SPAN_COUNT * 40;
pub const CAMERA_SANITY_FACTOR: f32 = 1.25;
pub const MOVE_VELOCITY_FACTOR: f32 = 20.0;

pub const MINIMUM_BLOCK_AMOUNT: u32 = 32;

pub const DEFAULT_NEW_LN_TEXT: &str = "Type in a lightning address!";
pub const DEFAULT_NEW_COLOR_TEXT: &str = "Random Color, if not specified.";
pub const DEFAULT_NEW_MESSAGE_TEXT: &str = "Leave an optional message for others to see";
pub const DEFAULT_HEIGHT_INPUT_TEXT: &str = "Type in a block height like 420 or 696969";

pub const THRESHOLD_FOR_PUSHBACK: i32 = -5_0000;

pub const DEFAULT_NO_PICK_COLOR: Srgba = Srgba {
    red: 0.7,
    green: 0.7,
    blue: 0.7,
    alpha: 1.0,
};

pub const ZOOM_IN_MAX: f32 = 0.25;
pub const TEXT_ZOOM_OUT_MAX: f32 = 3.0;
pub const BUILDING_ZOOM_OUT_MAX: f32 = 7.0;
pub const ZOOM_OUT_MAX: f32 = 12.0;

pub const DARKEST_BUILDING: Srgba = Srgba {
    red: 0.2,
    green: 0.2,
    blue: 0.2,
    alpha: 1.0,
};

pub const MAX_SELECTION_SIZE: usize = 500;
pub const MAX_MESSAGE_SIZE: usize = 140;
pub const ACCEPTABLE_CHARS: &str =
    "1234567890=⌫!#$%*&'@()[]+-_,.:;?ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz⇧⬆/\" ";

pub const UI_SMALL_TEXT_SIZE: f32 = 15.0;
pub const UI_MEDIUM_TEXT_SIZE: f32 = 20.0;
pub const UI_LARGE_TEXT_SIZE: f32 = 30.0;
pub const UI_LARGE_BUTTON_WIDTH: f32 = 75.0;
pub const UI_LARGE_BUTTON_HEIGHT: f32 = 45.0;

pub const UI_ICON_SIZE: f32 = 60.0;
pub const INDEX_WHITE_LAND: usize = 35;
pub const INDEX_MAX_LAND: usize = 34;

pub const DEFAULT_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
