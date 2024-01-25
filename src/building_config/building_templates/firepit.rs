use bevy::prelude::*;

use crate::{
    componenty::{AnimationIndices, AnimationTimer, BuildingStructure, Location},
    consty::TILE_SCALE,
};

use super::utils::process_scale;

#[allow(clippy::too_many_arguments)]
pub fn spawn_firepit(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    locationcoord: Location,
    visibility_toggle: Visibility,
    translation: Vec3,
    scale_modifier: Option<f32>,
) {
    let scale_modifier = process_scale(scale_modifier);
    let animation_indices = AnimationIndices { first: 9, last: 11 };
    builder.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: animation_indices.first,
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
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        BuildingStructure::FirePit,
        locationcoord,
    ));
}
