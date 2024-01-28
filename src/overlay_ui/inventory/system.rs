use bevy::prelude::*;

use crate::{
    eventy::DespawnInventoryHeights,
    resourcey::{ColorPalette, UserInventoryBlocks},
};

use super::{
    component::{InnerInventoryNode, InventoryNode},
    event::AddInventoryRow,
    layout::spawn_inventory_row_c,
};

#[allow(clippy::too_many_arguments)]
pub fn inventory_adder_system(
    mut commands: Commands,
    mut inventory_ent: Query<(Entity, &mut Visibility), With<InventoryNode>>,
    mut event: EventReader<AddInventoryRow>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    //tile_cart_vec: Res<TileCartVec>,
    mut inner_inv: Query<(Entity, &InnerInventoryNode)>,
    mut inventory: ResMut<UserInventoryBlocks>,
) {
    for inventory_event in event.read() {
        info!("inv event trig");
        for (ent, mut visi) in inventory_ent.iter_mut() {
            info!("entity found");
            *visi = Visibility::Visible;
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            for tile in &inventory_event.0 {
                inventory.ownedblocks.insert(tile.height, tile.clone());
                // despawn old if exist
                for (ent, node) in inner_inv.iter_mut() {
                    if node.0 == tile.height {
                        commands.entity(ent).despawn_recursive();
                    }
                }

                // spawn new inv row
                let child =
                    spawn_inventory_row_c(&mut commands, tile, font.clone(), colors.clone());

                commands.entity(ent).add_child(child);
            }
        }
    }
}

pub fn inventory_remover_system(
    mut event: EventReader<DespawnInventoryHeights>,
    mut commands: Commands,
    mut inner_inv: Query<(Entity, &InnerInventoryNode)>,
    mut inventory: ResMut<UserInventoryBlocks>,
) {
    for heights in event.read() {
        info!("despawn heights {:#?}", heights);

        for height in &heights.0 {
            for (ent, node) in inner_inv.iter_mut() {
                if &node.0 == height {
                    commands.entity(ent).despawn_recursive();
                }
            }
            inventory.ownedblocks.remove(height);
        }
    }
}
