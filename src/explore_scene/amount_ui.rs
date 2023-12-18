use bevy::prelude::*;
use rand::Rng;

use crate::{
    componenty::{AmountSelectedNode, AmountSelectedText, Location, Selected},
    consty::MINIMUM_BLOCK_AMOUNT,
    eventy::UpdateUiAmount,
    resourcey::{TileCart, TileCartData, TileMap},
};

pub fn setup_amount_selected_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            AmountSelectedNode,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                AmountSelectedText(0),
            ));
        });
}

pub fn update_amount_selected_text(
    mut event: EventReader<UpdateUiAmount>,
    selected_lands: Query<&Location, With<Selected>>,
    mut amount_selected_text: Query<&mut Text, With<AmountSelectedText>>,
    tile_map: ResMut<TileMap>,
    mut tile_cart: ResMut<TileCart>,
) {
    for _e in event.read() {
        let mut total_cost: u32 = 0;
        tile_cart.map.clear();
        for land in selected_lands.iter() {
            let a = tile_map.map.get(&land.ulam);
            let mut rng = rand::thread_rng();
            let r: f32 = rng.gen_range(0.0..1.0);
            let g: f32 = rng.gen_range(0.0..1.0);
            let b: f32 = rng.gen_range(0.0..1.0);
            let new_color = Color::Rgba {
                red: r,
                green: g,
                blue: b,
                alpha: 1.0,
            };
            if let Some(val) = a {
                total_cost += val.cost;
                tile_cart.map.insert(
                    land.ulam,
                    TileCartData {
                        event_date: Some(val.event_date),
                        ln_address: val.ln_address.clone(),
                        username: val.username.clone(),
                        color: Some(val.color),
                        message: val.message.clone(),
                        value: val.value,
                        cost: val.cost,
                        height: val.height,
                        new_ln_address: "".to_string(),
                        new_username: "".to_string(),
                        new_color,
                        new_message: "".to_string(),
                    },
                );
            } else {
                //info!("{}", land.ulam);
                total_cost += MINIMUM_BLOCK_AMOUNT;
                tile_cart.map.insert(
                    land.ulam,
                    TileCartData {
                        event_date: None,
                        ln_address: "".to_string(),
                        username: "".to_string(),
                        color: None,
                        message: "".to_string(),
                        value: 0,
                        cost: MINIMUM_BLOCK_AMOUNT,
                        height: land.ulam,
                        new_ln_address: "".to_string(),
                        new_username: "".to_string(),
                        new_color,
                        new_message: "".to_string(),
                    },
                );
            }
        }
        for mut text in amount_selected_text.iter_mut() {
            if total_cost == 0 {
                text.sections[0].value = "".to_string();
            } else {
                text.sections[0].value = format!("Total: {} sats", total_cost);
            }
        }
    }
}
