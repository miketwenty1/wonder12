use bevy::prelude::*;

use crate::{
    componenty::{BuildingStructure, Location},
    consty::TILE_SCALE,
};

use super::utils::process_scale;

#[allow(clippy::too_many_arguments)]
pub fn spawn_waterwell(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
    visibility_toggle: Visibility,
    translation: Vec3,
    scale_modifier: Option<f32>,
    offset: usize,
) {
    let scale_modifier = process_scale(scale_modifier);
    builder.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 15 + offset,
            },
            sprite: Sprite {
                color,
                ..Default::default()
            },
            transform: Transform {
                translation,
                scale: Vec3::new(
                    scale_modifier / TILE_SCALE,
                    scale_modifier / TILE_SCALE,
                    1.0,
                ),
                ..Default::default()
            },
            texture: texture.clone(),
            visibility: visibility_toggle,
            ..Default::default()
        },
        BuildingStructure::Waterwell,
        locationcoord,
    ));
}
