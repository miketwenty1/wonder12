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

#[derive(Component)]
pub struct UiToggle;

#[derive(Component)]
pub struct ToggleParent;

#[derive(Component)]
pub struct ToggleButton;

#[derive(Component)]
pub struct HideBuilding;

#[derive(Component)]
pub struct ShowBuilding;

#[derive(Component)]
pub struct ShowColors;

#[derive(Component)]
pub struct HideColors;

#[derive(Component)]
pub struct ShowValues;

#[derive(Component)]
pub struct ShowHeights;

#[derive(Component)]
pub struct ShowText;

#[derive(Component)]
pub struct HideText;

#[derive(Component)]
pub struct HideBuildingText;

#[derive(Component)]
pub struct ShowBuildingText;

#[derive(Component)]
pub struct ShowColorsText;

#[derive(Component)]
pub struct HideColorsText;

#[derive(Component)]
pub struct ShowValuesText;

#[derive(Component)]
pub struct ShowHeightsText;

#[derive(Component)]
pub struct ShowTextText;

#[derive(Component)]
pub struct HideTextText;

#[derive(Component)]
pub struct Toggle1Btn;

#[derive(Component)]
pub struct Toggle2Btn;

#[derive(Component)]
pub struct Toggle3Btn;

#[derive(Component)]
pub struct Toggle4Btn;

#[derive(Component)]
pub struct Toggle1BtnText;

#[derive(Component)]
pub struct Toggle2BtnText;

#[derive(Component)]
pub struct Toggle3BtnText;

#[derive(Component)]
pub struct Toggle4BtnText;

#[derive(Component)]
pub struct TileText;

#[derive(Component)]
pub struct AmountSelectedNode;

#[derive(Component)]
pub struct AmountSelectedText(pub u32);
