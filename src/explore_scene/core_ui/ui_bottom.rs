use bevy::{prelude::*, text::FontSmoothing};

use crate::{
    componenty::{
        BuySelectionButton, ClearSelectionButton, SelectedTileUi, UiInteractionBtn,
        UiOverlayingExplorerButton, ZoomInButton, ZoomOutButton,
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
                Button,
                Node {
                    width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                    height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors.button_color.into()),
                BorderColor(colors.lite_button_color),
                Visibility::Hidden,
                UiInteractionBtn,
                ClearSelectionButton,
                SelectedTileUi,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Clear"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                ));
            })
            .set_parent(parent_node);

        commands
            .spawn((
                Button,
                Node {
                    width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                    height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors.button_color.into()),
                BorderColor(colors.lite_button_color),
                UiInteractionBtn,
                ZoomOutButton,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("-"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_LARGE_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                ));
            })
            .set_parent(parent_node);
        commands
            .spawn((
                Button,
                Node {
                    width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                    height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(colors.lite_button_color),
                BackgroundColor(colors.button_color.into()),
                UiInteractionBtn,
                ZoomInButton,
                UiOverlayingExplorerButton,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("+"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_LARGE_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                ));
            })
            .set_parent(parent_node);
        commands
            .spawn((
                Button,
                Node {
                    width: Val::Px(UI_LARGE_BUTTON_WIDTH),
                    height: Val::Px(UI_LARGE_BUTTON_HEIGHT),
                    border: UiRect::all(Val::Px(2.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(colors.button_color.into()),
                Visibility::Hidden,
                BorderColor(colors.lite_button_color),
                UiInteractionBtn,
                UiOverlayingExplorerButton,
                BuySelectionButton,
                SelectedTileUi,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Buy"),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: UI_MEDIUM_TEXT_SIZE,
                        font_smoothing: FontSmoothing::AntiAliased,
                    },
                    TextColor(colors.text_color),
                ));
            })
            .set_parent(parent_node);
    }
}
