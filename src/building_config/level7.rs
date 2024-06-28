use bevy::prelude::*;
use rand::Rng;

use crate::componenty::Location;

use super::building_templates::{firepit::spawn_firepit, hut::spawn_hut, road::spawn_road};

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
    let x: f32 = rng.gen_range(0.0..5.0);
    let y: f32 = rng.gen_range(3.0..7.0);

    spawn_hut(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 3.0),
        Some(1.4),
    );

    let x: f32 = rng.gen_range(-10.0..-3.0);
    let y: f32 = rng.gen_range(-10.0..-1.0);

    spawn_hut(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 3.0),
        Some(0.85),
    );
    let x: f32 = rng.gen_range(1.0..5.0);
    let y: f32 = rng.gen_range(-7.0..-4.0);

    spawn_firepit(
        texture,
        layout,
        builder,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 4.0),
        Some(0.66),
    );

    spawn_road(
        texture,
        layout,
        builder,
        Color::rgba(1.0, 1.0, 1.0, 1.0),
        locationcoord,
        3,
        visibility_toggle,
    );
}
