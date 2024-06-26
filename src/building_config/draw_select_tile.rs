use bevy::prelude::*;

use crate::componenty::{DrawSelected, Location, Selected};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    locationcoord: Location,
    color: Color,
) {
    builder.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: 0,
            },
            sprite: Sprite {
                color,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        },
        DrawSelected,
        Selected(color),
        locationcoord,
    ));
}
