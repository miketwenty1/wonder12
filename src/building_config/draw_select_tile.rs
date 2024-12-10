use bevy::prelude::*;

use crate::{
    componenty::{DrawSelected, Location, Selected},
    consty::INDEX_WHITE_LAND,
};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    locationcoord: Location,
    color: Color,
) {
    let transform = Transform {
        translation: Vec3::new(0., 0., 10.),
        scale: Vec3::new(1.0, 1.0, 1.0),
        ..Default::default()
    };
    builder.spawn((
        Sprite {
            color,
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: INDEX_WHITE_LAND,
            }),
            ..Default::default()
        },
        transform,
        DrawSelected,
        Selected(color),
        locationcoord,
    ));
}
