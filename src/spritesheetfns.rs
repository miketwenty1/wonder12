use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::resourcey::SpriteSheetBg;

pub fn setup_spritesheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
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

    commands.insert_resource(SpriteSheetBg {
        layout: bg_texture_atlas,
        texture: bg_texture,
    });
}
