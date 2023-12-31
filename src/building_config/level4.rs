use bevy::prelude::*;

use crate::{
    componenty::{AnimationIndices, AnimationTimer, BuildingStructure, Location},
    consty::TILE_SCALE,
};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<TextureAtlas>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
    visibility_toggle: Visibility,
) {
    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color,
                index: 19,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-9., -8., 4.),
                scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 2.0),
                ..Default::default()
            },
            visibility: visibility_toggle,
            ..Default::default()
        },
        BuildingStructure::Hut2,
        locationcoord,
    ));

    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color,
                index: 19,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(7.5, 6.5, 4.),
                scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 2.0),
                ..Default::default()
            },
            visibility: visibility_toggle,
            ..Default::default()
        },
        BuildingStructure::Hut2,
        locationcoord,
    ));

    let animation_indices = AnimationIndices { first: 8, last: 10 };
    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3::new(4.5, -6.5, 5.),
                scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 3.0),
                ..Default::default()
            },
            visibility: visibility_toggle,
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        BuildingStructure::FirePit,
        locationcoord,
    ));

    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color: Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                },
                index: 20,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-4.0, 4.5, 5.),
                scale: Vec3::new(1.0 / TILE_SCALE / 1.6, 1.0 / TILE_SCALE / 1.6, 3.0),
                ..Default::default()
            },
            visibility: visibility_toggle,
            ..Default::default()
        },
        //AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        BuildingStructure::Waterwell,
        locationcoord,
    ));

    crate::building_config::road::spawn(
        &texture.clone(),
        builder,
        Color::rgba(1.0, 1.0, 1.0, 1.0),
        locationcoord,
        4,
        visibility_toggle,
    );
}
