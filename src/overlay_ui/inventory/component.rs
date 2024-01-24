use bevy::ecs::component::Component;

#[derive(Component)]
pub struct InventoryNode;

#[derive(Component)]
pub struct InnerInventoryNode(pub u32);

#[derive(Component)]
pub struct InventoryColorBox;

#[derive(Component)]
pub struct InventoryHeight;
