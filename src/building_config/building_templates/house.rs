use bevy::prelude::*;

use crate::{
    componenty::{BuildingStructure, Location},
    consty::SCALE_FACTOR,
};

use super::utils::process_scale;

#[allow(clippy::too_many_arguments)]
pub fn spawn_house(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
    visibility_toggle: Visibility,
    translation: Vec3,
    scale_modifier: Option<f32>,
) {
    let scale_modifier = process_scale(scale_modifier);
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
            color,
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: 14,
            }),
            image: texture.clone(),
            ..Default::default()
        },
        visibility_toggle,
        transform,
        BuildingStructure::House,
        locationcoord,
    ));
}
