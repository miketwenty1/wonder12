use bevy::color::Srgba;

use crate::consty::DARKEST_BUILDING;

pub fn sanitize_building_color(c: Srgba) -> Srgba {
    if c.red < DARKEST_BUILDING.red
        && c.green < DARKEST_BUILDING.green
        && c.blue < DARKEST_BUILDING.blue
    {
        return DARKEST_BUILDING;
    }
    c
}
