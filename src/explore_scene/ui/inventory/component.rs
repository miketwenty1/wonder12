use bevy::ecs::component::Component;

// overall inventory node
#[derive(Component)]
pub struct InventoryNode;

// left side column text
#[derive(Component)]
pub struct InventoryHeightText(pub u32);

// left side column nodes
#[derive(Component)]
pub struct InventoryHeightTextNode(pub u32);

// right side column buttons for blocks
#[derive(Component)]
pub struct InventoryColorBox(pub u32);

// right side column nodes for colored buttons
#[derive(Component)]
pub struct InventoryColorBoxNode(pub u32);

// where the block grid is of individual blocks
#[derive(Component)]
pub struct InventoryRowsNode;

#[derive(Component)]
pub struct PlaceHolderInventoryNode;

#[derive(Component)]
pub struct InventoryToggleButton;

#[derive(Component)]
pub struct InventoryToggleable;
