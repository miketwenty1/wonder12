use crate::consty::{DARKEST_BUILDING, LIGHTEST_TEXT};
use bevy::color::{Color, Srgba};
pub fn sanitize_building_color(c: Srgba) -> Srgba {
    if c.red < DARKEST_BUILDING.red
        && c.green < DARKEST_BUILDING.green
        && c.blue < DARKEST_BUILDING.blue
    {
        return DARKEST_BUILDING;
    }
    c
}

pub fn get_text_color(c: &Color) -> Color {
    if c.to_srgba().red > LIGHTEST_TEXT.red
        && c.to_srgba().green > LIGHTEST_TEXT.green
        && c.to_srgba().blue > LIGHTEST_TEXT.blue
    {
        Color::Srgba(Srgba::BLACK)
    } else {
        Color::Srgba(Srgba::WHITE)
    }
}
