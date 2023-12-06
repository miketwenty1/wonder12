use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct UiNode;

#[derive(Component)]
pub struct UiTileSelectedButton;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    pub ulam: u32,
    pub quad: ulam::Quad,
    pub selected: bool,
}

#[derive(Component)]
pub struct ClearSelectionButton;

#[derive(Component)]
pub struct DetailSelectionButton;

#[derive(Component)]
pub struct ZoomOutButton;

#[derive(Component)]
pub struct ZoomInButton;

#[derive(Component, Clone, Copy)]
pub struct Land;

#[derive(Component, Clone, Copy)]
pub enum BuildingStructure {
    //Empty,
    Hut,
    DirtRoad,
    //DirtRoadCorner,
    //DirtRoad2,
    //DirtRoadCorner2,
    FirePit,
}
