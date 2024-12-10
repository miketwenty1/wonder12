use bevy::{prelude::*, text::FontSmoothing};

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
                Node {
                    padding: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(4.0),
                        bottom: Val::Px(4.0),
                    },
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Visibility::Hidden,
                BackgroundColor(colors.button_color.into()),
                BorderRadius::all(Val::Px(8.0)),
                BlockCountNode,
                SelectedTileUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                    BlockCountText,
                ));
            })
            .set_parent(parent_node);

        let _block_count_node = commands
            .spawn((
                Node {
                    padding: UiRect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        top: Val::Px(4.0),
                        bottom: Val::Px(4.0),
                    },
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                Visibility::Hidden,
                BackgroundColor(colors.button_color.into()),
                BorderRadius::all(Val::Px(8.0)),
                SelectedTileUi,
                AmountNode,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    AmountText,
                ));
            })
            .set_parent(parent_node);
    }
}
