use bevy::{
    prelude::*,
    render::texture::{ImageLoaderSettings, ImageSampler},
};

use crate::resourcey::{SpriteSheetBuilding, SpriteSheetLand, SpriteSheetSelect};

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
    // let bg_texture = asset_server.load_with_settings(
    //     "spritesheet/grassdirtbg.png",
    //     |settings: &mut ImageLoaderSettings| {
    //         settings.sampler = ImageSampler::nearest();
    //     },
    // );
    let select_texture = asset_server.load_with_settings(
        "spritesheet/select.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );
    let land_texture = asset_server.load_with_settings(
        "spritesheet/ss-land-v12.png",
        |settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        },
    );

    let building_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        18,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let building_texture_atlas = texture_atlases.add(building_atlas);

    // let bg_atlas = TextureAtlasLayout::from_grid(
    //     UVec2::new(32, 32),
    //     12,
    //     1,
    //     Some(UVec2::new(2, 2)),
    //     Some(bevy::prelude::UVec2::new(1, 1)),
    // );
    // let bg_texture_atlas = texture_atlases.add(bg_atlas);

    let select_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        8,
        1,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let select_texture_atlas = texture_atlases.add(select_atlas);

    let land_atlas = TextureAtlasLayout::from_grid(
        bevy::prelude::UVec2::new(32, 32),
        6,
        7,
        Some(bevy::prelude::UVec2::new(2, 2)),
        Some(bevy::prelude::UVec2::new(1, 1)),
    );
    let land_texture_atlas = texture_atlases.add(land_atlas);

    commands.insert_resource(SpriteSheetBuilding {
        layout: building_texture_atlas,
        texture: building_texture,
    });
    // commands.insert_resource(SpriteSheetBg {
    //     layout: bg_texture_atlas,
    //     texture: bg_texture,
    // });
    commands.insert_resource(SpriteSheetSelect {
        layout: select_texture_atlas,
        texture: select_texture,
    });
    commands.insert_resource(SpriteSheetLand {
        layout: land_texture_atlas,
        texture: land_texture,
    });
}
