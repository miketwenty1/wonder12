use crate::consty::DARKEST_BUILDING;
use bevy::render::color::Color;

pub fn sanitize_building_color(c: Color) -> Color {
    if c.r() < DARKEST_BUILDING.r() && c.g() < DARKEST_BUILDING.g() && c.b() < DARKEST_BUILDING.b()
    {
        return DARKEST_BUILDING;
    }
    c
}
