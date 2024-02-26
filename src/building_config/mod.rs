use bevy::prelude::*;

use crate::componenty::Location;

pub mod level1;
pub mod level10;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;

pub mod select_tile;
pub mod utils;

pub mod building_templates;

pub fn spawn_tile_level(
    building_sprite_index: usize,
    layout: &Handle<TextureAtlasLayout>,
    texture: &Handle<Image>,
    builder: &mut ChildBuilder,
    color_for_sprites: LegacyColor,
    locationcoord: Location,
    building_visibility_toggle: Visibility,
) {
    match building_sprite_index {
        1 => {
            level1::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        2 => {
            level2::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        3 => {
            level3::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        4 => {
            level4::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        5 => {
            level5::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        6 => {
            level6::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        7 => {
            level7::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        8 => {
            level8::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        9 => {
            level9::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        10 => {
            level10::spawn(
                texture,
                layout,
                builder,
                color_for_sprites,
                locationcoord,
                building_visibility_toggle,
            );
        }
        100 => {
            select_tile::spawn(texture, layout, builder, locationcoord);
        }
        _ => {
            // do nothing
        }
    }
}
