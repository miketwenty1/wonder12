use bevy::render::color::LegacyColor;

use crate::consty::DARKEST_BUILDING;


pub fn sanitize_building_color(c: LegacyColor) -> LegacyColor {
    if c.r() < DARKEST_BUILDING.r() && c.g() < DARKEST_BUILDING.g() && c.b() < DARKEST_BUILDING.b()
    {
        return DARKEST_BUILDING;
    }
    c
}
