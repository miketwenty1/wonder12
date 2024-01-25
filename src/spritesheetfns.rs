use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::resourcey::{SpriteSheetBg, SpriteSheetBuilding, SpriteSheetSelect};

pub fn setup_spritesheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let building_texture = asset_server.load_with_settings(
        "spritesheet/buildings1.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );
    let building_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        18,
        1,
        Some(Vec2::new(2.0, 2.0)),
        Some(Vec2::new(1.0, 1.0)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    let bg_texture = asset_server.load_with_settings(
        "spritesheet/grassdirtbg.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::linear();
        },
    );
    let bg_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        12,
        1,
        Some(Vec2::new(2.0, 2.0)),
        Some(Vec2::new(1.0, 1.0)),
    );
    let bg_texture_atlas = texture_atlases.add(bg_atlas);

    let select_texture = asset_server.load_with_settings(
        "spritesheet/select.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );
    let select_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(32.0, 32.0),
        8,
        1,
        Some(Vec2::new(2.0, 2.0)),
        Some(Vec2::new(1.0, 1.0)),
    );
    let select_texture_atlas = texture_atlases.add(select_atlas);

    commands.insert_resource(SpriteSheetBuilding {
        layout: building_texture_atlas,
        texture: building_texture,
    });
    commands.insert_resource(SpriteSheetBg {
        layout: bg_texture_atlas,
        texture: bg_texture,
    });
    commands.insert_resource(SpriteSheetSelect {
        layout: select_texture_atlas,
        texture: select_texture,
    });
}
