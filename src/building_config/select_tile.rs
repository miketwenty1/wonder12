use bevy::prelude::*;

use crate::{
    componenty::{AnimationIndices, AnimationTimer, Location, ManualSelected, Selected},
    utils::get_random_color,
};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    locationcoord: Location,
) {
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    //info!("spawn select");
    builder.spawn((
        SpriteSheetBundle {
            atlas: TextureAtlas {
                layout: layout.clone(),
                index: animation_indices.first,
            },

            transform: Transform {
                translation: Vec3::new(0., 0., 10.),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            texture: texture.clone(),
            ..Default::default()
        },
        ManualSelected,
        Selected(bevy::prelude::Color::Srgba(get_random_color())),
        locationcoord,
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
    ));
}
