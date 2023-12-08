use bevy::render::color::Color;

pub const CHUNK_PIXEL_SIZE: f32 = 400.0;
pub const TILE_SCALE: f32 = 3.0;
pub const TILE_PIXEL_SIZE: f32 = 32.0;
//const TILE_PADDING_SIZE: f32 = 0.0;
pub const TOTAL_TILE_SCALE_SIZE: f32 = TILE_PIXEL_SIZE * TILE_SCALE + 4.0;
pub const CHUNK_TILE_SPAN_COUNT: i32 = (CHUNK_PIXEL_SIZE / TOTAL_TILE_SCALE_SIZE) as i32;
pub const DESPAWN_TILE_THRESHOLD: i32 = 51 + CHUNK_TILE_SPAN_COUNT * 2;
pub const CAMERA_SANITY_FACTOR: f32 = 1.25;
pub const MOVE_VELOCITY_FACTOR: f32 = 20.0;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub const MINIMUM_BLOCK_AMOUNT: u32 = 128;
