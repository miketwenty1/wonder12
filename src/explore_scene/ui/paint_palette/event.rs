use bevy::{prelude::Event, render::color::Color};

#[derive(Event, Debug)]
pub struct NewColorPicked(pub Color);

#[derive(Event, Debug)]
pub struct DrawSelectTileEvent(pub i32, pub i32, pub Color);
