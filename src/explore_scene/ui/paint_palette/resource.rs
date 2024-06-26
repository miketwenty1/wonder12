use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct MovementPaletteSelected(pub bool);

#[derive(Resource, Clone, Debug)]
pub struct DefaultDrawColorPalette {
    pub colors: Vec<Color>,
}
