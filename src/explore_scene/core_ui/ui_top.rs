use bevy::prelude::*;

use crate::{componenty::SelectedTileUi, consty::UI_MEDIUM_TEXT_SIZE, resourcey::ColorPalette};

use super::components::{
    AmountNode, AmountText, BlockCountNode, BlockCountText, ExplorerUiNodeTop,
};

pub fn top_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeTop>>,
) {
    for parent_node in placement_query.iter() {
        let _blocks_selected_node = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: Val::Px(4.0),
                            bottom: Val::Px(4.0),
                        },
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                BlockCountNode,
                SelectedTileUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        //Blocks Selected: 0
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: UI_MEDIUM_TEXT_SIZE,
                            color: colors.text_color,
                        },
                    ),
                    BlockCountText,
                ));
            })
            .set_parent(parent_node);

        let _block_count_node = commands
            .spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: Val::Px(4.0),
                            bottom: Val::Px(4.0),
                        },
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(8.0)),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                SelectedTileUi,
                AmountNode,
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        //Price: 0 satoshis
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: UI_MEDIUM_TEXT_SIZE,
                            color: colors.text_color,
                        },
                    ),
                    AmountText,
                ));
            })
            .set_parent(parent_node);
    }
}
