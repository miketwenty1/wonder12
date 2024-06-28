use bevy::prelude::*;

use crate::componenty::Location;

#[allow(clippy::too_many_arguments)]
pub fn spawn(
    _texture: &Handle<Image>,
    _layout: &Handle<TextureAtlasLayout>,
    _builder: &mut ChildBuilder,
    _color: Color,
    _locationcoord: Location,
    _visibility_toggle: Visibility,
) {
}
