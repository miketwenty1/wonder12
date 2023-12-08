use bevy::prelude::*;

use crate::componenty::{AnimationIndices, AnimationTimer, Location, Selected};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<TextureAtlas>,
    builder: &mut ChildBuilder,
    _color: Color,
    locationcoord: Location,
) {
    let animation_indices = AnimationIndices { first: 9, last: 16 };
    //info!("spawn select");
    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            // sprite: TextureAtlasSprite {
            //     color,
            //     index: 9,
            //     ..Default::default()
            // },
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Selected,
        locationcoord,
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
    ));
}
