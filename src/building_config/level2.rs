use bevy::prelude::*;

use crate::{
    componenty::{BuildingStructure, Location},
    consty::TILE_SCALE,
};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<TextureAtlas>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
) {
    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color,
                index: 1,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(5., 0., 4.),
                scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        BuildingStructure::Hut,
        locationcoord,
    ));

    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color,
                index: 1,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-5., -5., 4.),
                scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        BuildingStructure::Hut,
        locationcoord,
    ));

    crate::building_config::road::spawn(
        &texture.clone(),
        builder,
        Color::rgba(1.0, 1.0, 1.0, 1.0),
        locationcoord,
        0,
    );
}
