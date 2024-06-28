use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct DefaultDrawColorPalette {
    pub colors: Vec<Color>,
}

#[derive(Resource, Clone, Debug)]
pub struct ViewablePaletteTiles(pub bool);
