use crate::{BuildingStructure, Location, TILE_SCALE};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<TextureAtlas>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
) {
    info!("roadbuilding");
    // index number on sprite sheet and rotation degree. seems like 90 degrees is = 1.565
    let road: (usize, f32) = match locationcoord.quad {
        ulam::Quad::North => (2, 0.0),
        ulam::Quad::NorthEast => (3, 1.565),
        ulam::Quad::East => (2, 1.565),
        ulam::Quad::SouthEast => (3, 0.0),
        ulam::Quad::South => (2, 0.0),
        ulam::Quad::SouthWest => (3, 4.695),
        ulam::Quad::West => (2, 1.56),
        ulam::Quad::NorthWest => (3, 3.13),
        ulam::Quad::Center => (2, 0.0),
    };
    builder.spawn((
        SpriteSheetBundle {
            texture_atlas: texture.clone(),
            sprite: TextureAtlasSprite {
                color,
                index: road.0,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 2.),
                // scale: Vec3::new(TILE_SCALE, TILE_SCALE, 1.0),
                rotation: Quat::from_rotation_z(road.1),
                ..Default::default()
            },
            ..Default::default()
        },
        BuildingStructure::Road,
        locationcoord,
    ));
}
