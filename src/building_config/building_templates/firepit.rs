use bevy::prelude::*;

use crate::{
    componenty::{AnimationIndices, AnimationTimer, BuildingStructure, Location},
    consty::SCALE_FACTOR,
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
    let transform = Transform {
        translation,
        scale: Vec3::new(
            scale_modifier / SCALE_FACTOR,
            scale_modifier / SCALE_FACTOR,
            1.0,
        ),
        ..Default::default()
    };

    builder.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: animation_indices.first,
            }),
            ..Default::default()
        },
        visibility_toggle,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        BuildingStructure::FirePit,
        locationcoord,
        animation_indices,
    ));
}
