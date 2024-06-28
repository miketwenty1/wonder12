use bevy::prelude::*;

use crate::componenty::{BuildingStructure, Location};

const RADIAN_90: f32 = 1.5707961;
#[allow(clippy::too_many_arguments)]
pub fn spawn_road(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
    offset: usize,
    visibility: Visibility,
) {
    //info!("roadbuilding");
    // index number on sprite sheet and rotation degree. seems like 90 degrees is = 1.5707961
    let road: (usize, f32) = match locationcoord.quad {
        ulam::Quad::North => (2, 0.0),
        ulam::Quad::NorthEast => (3, RADIAN_90),
        ulam::Quad::East => (2, RADIAN_90),
        ulam::Quad::SouthEast => (3, 0.0),
        ulam::Quad::South => (2, 0.0),
        ulam::Quad::SouthWest => (3, RADIAN_90 * 3.),
        ulam::Quad::West => (2, RADIAN_90),
        ulam::Quad::NorthWest => (3, RADIAN_90 * 2.),
        ulam::Quad::Center => (2, 0.0),
    };
    builder.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 2.),
                // scale: Vec3::new(TILE_SCALE, TILE_SCALE, 1.0),
                rotation: Quat::from_rotation_z(road.1),
                ..Default::default()
            },
            texture: texture.clone(),
            visibility,
            ..Default::default()
        },
        BuildingStructure::Road,
        locationcoord,
        TextureAtlas {
            layout: layout.clone(),
            index: road.0 + offset,
        },
    ));
}
