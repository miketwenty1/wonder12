use bevy::ecs::component::Component;

// overall inventory node
#[derive(Component)]
pub struct PaintPaletteNode;

#[derive(Component)]
pub struct PaletteBtn;

#[derive(Component)]
pub struct PaletteMoveBtn;

#[derive(Component)]
pub struct PaletteEraserBtn;

#[derive(Component)]
pub struct PaletteEyedropBtn;

#[derive(Component)]
pub struct PaletteTrashBtn;

#[derive(Component)]
pub struct ColorPaletteViewText;

#[derive(Component)]
pub struct ColorPaletteViewTextNode;

#[derive(Component)]
pub struct AddToCustomPaletteBtn;
