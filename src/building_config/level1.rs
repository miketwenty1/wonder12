use bevy::prelude::*;
use rand::Rng;

use crate::componenty::Location;

use super::building_templates::camp::spawn_camp;

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
    let x: f32 = rng.gen_range(-12.0..12.0);
    let y: f32 = rng.gen_range(-12.0..12.0);

    spawn_camp(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(x, y, 3.0),
        Some(0.75),
    );
}
