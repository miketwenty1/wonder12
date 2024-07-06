use bevy::prelude::*;

use crate::{
    componenty::{Location, Selected, SelectedTileUi},
    consty::MINIMUM_BLOCK_AMOUNT,
    eventy::UpdateUiAmount,
    resourcey::{TileCart, TileCartData, UserPurchasedBlockMessage, WorldOwnedTileMap},
};

use super::core_ui::components::{AmountText, BlockCountText};

#[allow(clippy::type_complexity)]
pub fn update_amount_selected_text(
    mut event: EventReader<UpdateUiAmount>,

    selected_lands: Query<(&Location, &Selected), With<Selected>>,
    mut amount_selected_text: Query<
        &mut Text,
        (
            With<AmountText>,
            Without<BlockCountText>,
            Without<SelectedTileUi>,
        ),
    >,
    mut block_count_text: Query<
        &mut Text,
        (
            With<BlockCountText>,
            Without<AmountText>,
            Without<SelectedTileUi>,
        ),
    >,
    tile_map: Res<WorldOwnedTileMap>,
    mut tile_cart: ResMut<TileCart>,
    mut tile_selected_button_q: Query<
        &mut Visibility,
        (
            With<SelectedTileUi>,
            Without<AmountText>,
            Without<BlockCountText>,
        ),
    >,
) {
    for _e in event.read() {
        let mut total_cost: u32 = 0;
        tile_cart.map.clear();
        for (land, selected) in selected_lands.iter() {
            // info!("raw query {:?}", selected.0);
            let a = tile_map.map.get(&land.ulam);
            // info!("selected color: {:?}", selected.0);
            // land exist and is owned by someone
            if let Some(val) = a {
                //info!("land exist and is owned by someone...");
                let current_message = UserPurchasedBlockMessage {
                    username: val.username.clone(),
                    value: val.value,
                    message: val.message.clone(),
                };
                total_cost += val.cost;

                //info!("selected color is----->: {:?}", selected.0);
                tile_cart.map.insert(
                    land.ulam,
                    TileCartData {
                        event_date: Some(val.event_date),
                        ln_address: val.ln_address.clone(),
                        username: val.username.clone(),
                        color: Some(val.color),
                        messages: vec![current_message].into(),
                        value: val.value,
                        cost: val.cost,
                        height: val.height,
                        new_ln_address: "".to_string(),
                        new_username: "".to_string(),
                        new_color: selected.0,
                        new_color_text: "".to_string(),
                        new_message: "".to_string(),
                    },
                );
            // this is a new land that hasn't been purchased yet
            } else {
                //info!("selected color is++++++> {:?}", selected.0);

                total_cost += MINIMUM_BLOCK_AMOUNT;
                tile_cart.map.insert(
                    land.ulam,
                    TileCartData {
                        event_date: None,
                        ln_address: "".to_string(),
                        username: "".to_string(),
                        color: None,
                        messages: None,
                        value: 0,
                        cost: MINIMUM_BLOCK_AMOUNT,
                        height: land.ulam,
                        new_ln_address: "".to_string(),
                        new_username: "".to_string(),
                        new_color: selected.0,
                        new_color_text: "".to_string(),
                        new_message: "".to_string(),
                    },
                );
            }
        }
        for mut text in amount_selected_text.iter_mut() {
            if total_cost == 0 {
                text.sections[0].value = "".to_string();

                for mut visibility in tile_selected_button_q.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
            } else {
                for mut visibility in tile_selected_button_q.iter_mut() {
                    *visibility = Visibility::Visible;
                }
                text.sections[0].value = format!("Price: {} satoshis", total_cost);
            }
        }
        for mut text in block_count_text.iter_mut() {
            let selected_count = tile_cart.map.len();
            if selected_count == 0 {
                text.sections[0].value = "".to_string();
            } else {
                text.sections[0].value = format!("Blocks Selected: {}", selected_count);
            }
        }
    }
}
