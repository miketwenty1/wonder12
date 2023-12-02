use bevy::prelude::*;

use crate::{Location, Selected};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<TextureAtlas>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
) {
    info!("spawn select");
    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color,
                index: 9,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Selected,
        locationcoord,
    ));
}
