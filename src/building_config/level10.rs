use bevy::{color::palettes::css::WHITE, prelude::*};
use rand::Rng;

use crate::componenty::Location;

use super::building_templates::{house::spawn_house, road::spawn_road, waterwell::spawn_waterwell};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
    visibility_toggle: Visibility,
) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(-4.0..4.0);
    let y: f32 = rng.gen_range(5.0..7.0);

    spawn_house(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 3.0),
        Some(1.5),
    );

    let x: f32 = rng.gen_range(-9.0..9.0);
    let y: f32 = rng.gen_range(-9.0..-6.0);
    spawn_waterwell(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 4.0),
        Some(0.75),
        1,
    );

    spawn_road(
        texture,
        layout,
        builder,
        WHITE.into(),
        locationcoord,
        5,
        visibility_toggle,
    );
}
