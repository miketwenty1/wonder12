use bevy::prelude::*;

use crate::{
    componenty::{
        BuySelectionButton, ClearSelectionButton, SelectedTileUi, UiOverlayingExplorerButton,
        ZoomInButton, ZoomOutButton,
    },
    consty::{
        UI_LARGE_BUTTON_HEIGHT, UI_LARGE_BUTTON_WIDTH, UI_LARGE_TEXT_SIZE, UI_MEDIUM_TEXT_SIZE,
    },
    resourcey::ColorPalette,
};

use super::components::ExplorerUiNodeBottom;

pub fn bottom_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
    placement_query: Query<Entity, With<ExplorerUiNodeBottom>>,
) {
    for parent_node in placement_query.iter() {
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                        height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.lite_button_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                ClearSelectionButton,
                SelectedTileUi,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Clear",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        color: colors.text_color,
                    },
                ));
            })
            .set_parent(parent_node);

        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                        height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.lite_button_color),
                    background_color: colors.button_color.into(),
                    ..default()
                },
                ZoomOutButton,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "-",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_LARGE_TEXT_SIZE,
                        color: colors.text_color,
                    },
                ));
            })
            .set_parent(parent_node);
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                        height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.lite_button_color),
                    background_color: colors.button_color.into(),
                    ..default()
                },
                ZoomInButton,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "+",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_LARGE_TEXT_SIZE,
                        color: colors.text_color,
                    },
                ));
            })
            .set_parent(parent_node);
        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                        height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(colors.lite_button_color),
                    background_color: colors.button_color.into(),
                    visibility: Visibility::Hidden,
                    ..default()
                },
                UiOverlayingExplorerButton,
                BuySelectionButton,
                SelectedTileUi,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    "Buy",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        color: colors.text_color,
                    },
                ));
            })
            .set_parent(parent_node);
    }
}
