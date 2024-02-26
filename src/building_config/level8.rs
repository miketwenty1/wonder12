use bevy::prelude::*;
use rand::Rng;

use crate::componenty::Location;

use super::building_templates::{
    firepit::spawn_firepit, road::spawn_road, shack::spawn_shack, waterwell::spawn_waterwell,
};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: LegacyColor,
    locationcoord: Location,
    visibility_toggle: Visibility,
) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen_range(0.0..5.0);
    let y: f32 = rng.gen_range(2.0..4.0);

    spawn_shack(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 3.0),
        Some(1.5),
    );

    let x: f32 = rng.gen_range(2.0..7.0);
    let y: f32 = rng.gen_range(-9.0..-6.0);

    spawn_firepit(
        texture,
        layout,
        builder,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 4.0),
        Some(0.66),
    );

    let x: f32 = rng.gen_range(-8.0..-3.0);
    let y: f32 = rng.gen_range(-7.0..-3.0);

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
        LegacyColor::rgba(1.0, 1.0, 1.0, 1.0),
        locationcoord,
        5,
        visibility_toggle,
    );
}
