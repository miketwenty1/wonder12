use bevy::prelude::*;

use crate::{
    componenty::{AmountSelectedNode, AmountSelectedText, Location, Selected},
    consty::MINIMUM_BLOCK_AMOUNT,
    eventy::UpdateUiAmount,
    resourcey::TileMap,
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
) {
    for _e in event.read() {
        let mut total_amount: u32 = 0;
        for land in selected_lands.iter() {
            let a = tile_map.map.get(&land.ulam);
            if let Some(val) = a {
                total_amount += val.amount * 2;
            } else {
                total_amount += MINIMUM_BLOCK_AMOUNT;
            }
        }
        for mut text in amount_selected_text.iter_mut() {
            if total_amount == 0 {
                text.sections[0].value = "".to_string();
            } else {
                text.sections[0].value = format!("Total: {} sats", total_amount);
            }
        }
    }
}
