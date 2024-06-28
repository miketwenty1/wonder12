use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct DefaultDrawColorPalette {
    pub colors: Vec<Color>,
}
