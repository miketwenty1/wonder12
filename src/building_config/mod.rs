use bevy::prelude::*;

use crate::componenty::Location;

pub mod level1;
pub mod level2;
pub mod level3;
pub mod road;
pub mod select_tile;

pub fn spawn_tile_level(
    building_sprite_index: usize,
    texture_handle: &Handle<TextureAtlas>,
    builder: &mut ChildBuilder,
    color_for_sprites: Color,
    locationcoord: Location,
) {
    match building_sprite_index {
        1 => {
            level1::spawn(texture_handle, builder, color_for_sprites, locationcoord);
        }
        2 => {
            level2::spawn(texture_handle, builder, color_for_sprites, locationcoord);
        }
        3 => {
            level3::spawn(texture_handle, builder, color_for_sprites, locationcoord);
        }
        100 => {
            select_tile::spawn(texture_handle, builder, color_for_sprites, locationcoord);
        }
        _ => {
            // do nothing
        }
    }
}
