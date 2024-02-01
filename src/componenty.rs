use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    pub ulam: u32,
    pub quad: ulam::Quad,
    pub selected: bool,
}
#[derive(Component, Clone, Copy)]
pub struct Land;
