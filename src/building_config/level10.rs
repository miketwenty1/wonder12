use bevy::prelude::*;

use crate::componenty::Location;

use super::building_templates::{castle::spawn_castle, road::spawn_road};

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    texture: &Handle<Image>,
    layout: &Handle<TextureAtlasLayout>,
    builder: &mut ChildBuilder,
    color: Color,
    locationcoord: Location,
    visibility_toggle: Visibility,
) {
    spawn_castle(
        texture,
        layout,
        builder,
        color,
        locationcoord,
        visibility_toggle,
        Vec3::new(0., 0., 3.0),
        Some(2.0),
    );

    spawn_road(
        texture,
        layout,
        builder,
        Color::rgba(1.0, 1.0, 1.0, 1.0),
        locationcoord,
        5,
        visibility_toggle,
    );
}
