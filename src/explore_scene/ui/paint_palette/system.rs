use crate::{
    componenty::{Selected, UiInteractionBtn},
    eventy::ClearSelectionEvent,
    resourcey::{ColorPalette, UiInteracting},
};

use super::{
    component::{
        ColorPaletteViewTextNode, IndividualColorInPalette, PaletteEraserBtn, PaletteEyedropBtn,
        PaletteMoveBtn, PalettePencilBtn, PaletteTrashBtn, PaletteViewHideBtn, ViewHideImg,
    },
    event::{HideSelectedTiles, NewColorPicked, ViewSelectedTiles},
    resource::ViewablePaletteTiles,
    state::ToolPaletteUiState,
};
use bevy::{
    color::palettes::css::DARK_GRAY,
    input::{mouse::MouseButtonInput, touch::TouchPhase, ButtonState},
    prelude::*,
};

#[allow(clippy::type_complexity)]
pub fn ui_interaction_enabled_buttons(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<UiInteractionBtn>)>,
    mut ui_interacting: ResMut<UiInteracting>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //info!("interaction true");
                *ui_interacting = UiInteracting(true);
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn ui_interaction_released_buttons(
    mut ui_interacting: ResMut<UiInteracting>,
    mut mouse_event: EventReader<MouseButtonInput>,
    mut touch_event: EventReader<TouchInput>,
) {
    for mouse in mouse_event.read() {
        if mouse.button == MouseButton::Left && mouse.state == ButtonState::Released {
            //info!("interaction false");
            ui_interacting.0 = false;
        }
    }

    for touch in touch_event.read() {
        if touch.phase == TouchPhase::Ended || touch.phase == TouchPhase::Canceled {
            ui_interacting.0 = false;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn move_palette_button(
    mut move_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PaletteMoveBtn>),
    >,
    colors: Res<ColorPalette>,
    mut tool_palette_state_c: ResMut<NextState<ToolPaletteUiState>>,
    tool_palette_state: Res<State<ToolPaletteUiState>>,
    mut tool_set: ParamSet<(
        Query<&mut BackgroundColor, (With<PalettePencilBtn>, Without<PaletteMoveBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteEraserBtn>, Without<PaletteMoveBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteEyedropBtn>, Without<PaletteMoveBtn>)>,
    )>,
) {
    for (interaction, mut color) in &mut move_query {
        match *interaction {
            Interaction::Pressed => {
                tool_palette_state_c.set(ToolPaletteUiState::Move);
                *color = colors.green_color.into();

                for mut bg_color in tool_set.p0().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p1().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p2().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                if **tool_palette_state != ToolPaletteUiState::Move {
                    *color = colors.light_color.into();
                } else {
                    *color = colors.accent_color.into();
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn pencil_palette_button(
    mut pencil_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PalettePencilBtn>),
    >,
    colors: Res<ColorPalette>,
    mut tool_palette_state_c: ResMut<NextState<ToolPaletteUiState>>,
    tool_palette_state: Res<State<ToolPaletteUiState>>,
    mut tool_set: ParamSet<(
        Query<&mut BackgroundColor, (With<PaletteEraserBtn>, Without<PalettePencilBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteMoveBtn>, Without<PalettePencilBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteEyedropBtn>, Without<PalettePencilBtn>)>,
    )>,
    // mut view_event: EventWriter<ViewSelectedTiles>,
) {
    for (interaction, mut color) in &mut pencil_query {
        match *interaction {
            Interaction::Pressed => {
                tool_palette_state_c.set(ToolPaletteUiState::Pencil);
                *color = colors.green_color.into();
                for mut bg_color in tool_set.p0().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p1().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p2().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                if **tool_palette_state != ToolPaletteUiState::Pencil {
                    *color = colors.light_color.into();
                } else {
                    *color = colors.accent_color.into();
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn eraser_palette_button(
    mut eraser_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PaletteEraserBtn>),
    >,
    colors: Res<ColorPalette>,
    mut tool_palette_state_c: ResMut<NextState<ToolPaletteUiState>>,
    tool_palette_state: Res<State<ToolPaletteUiState>>,
    mut tool_set: ParamSet<(
        Query<&mut BackgroundColor, (With<PalettePencilBtn>, Without<PaletteEraserBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteMoveBtn>, Without<PaletteEraserBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteEyedropBtn>, Without<PaletteEraserBtn>)>,
    )>,
    mut view_event: EventWriter<ViewSelectedTiles>,
) {
    for (interaction, mut color) in &mut eraser_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.green_color.into();
                view_event.send(ViewSelectedTiles);
                tool_palette_state_c.set(ToolPaletteUiState::Eraser);

                for mut bg_color in tool_set.p0().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p1().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p2().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                if **tool_palette_state != ToolPaletteUiState::Eraser {
                    *color = colors.light_color.into();
                } else {
                    *color = colors.accent_color.into();
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn eyedrop_palette_button(
    mut eyedrop_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PaletteEyedropBtn>),
    >,
    colors: Res<ColorPalette>,
    mut tool_palette_state_c: ResMut<NextState<ToolPaletteUiState>>,
    tool_palette_state: Res<State<ToolPaletteUiState>>,
    mut tool_set: ParamSet<(
        Query<&mut BackgroundColor, (With<PalettePencilBtn>, Without<PaletteEyedropBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteMoveBtn>, Without<PaletteEyedropBtn>)>,
        Query<&mut BackgroundColor, (With<PaletteEraserBtn>, Without<PaletteEyedropBtn>)>,
    )>,
) {
    for (interaction, mut color) in &mut eyedrop_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.green_color.into();
                tool_palette_state_c.set(ToolPaletteUiState::Eyedrop);

                for mut bg_color in tool_set.p0().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p1().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
                for mut bg_color in tool_set.p2().iter_mut() {
                    *bg_color = colors.light_color.into();
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                if **tool_palette_state != ToolPaletteUiState::Eyedrop {
                    *color = colors.light_color.into();
                } else {
                    *color = colors.accent_color.into();
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn individual_color_palette_button(
    mut query: Query<
        (
            &Interaction,
            &mut BorderColor,
            &IndividualColorInPalette,
            &mut Style,
        ),
        (Changed<Interaction>, With<IndividualColorInPalette>),
    >,
    colors: Res<ColorPalette>,
    mut color_update: EventWriter<NewColorPicked>,
) {
    for (interaction, mut border_color, color, mut style) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                style.border = UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(2.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(2.0),
                };
                color_update.send(NewColorPicked(color.0));
            }
            Interaction::Hovered => {
                *border_color = colors.light_color.into();
                style.border = UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(0.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(0.0),
                };
            }
            Interaction::None => {
                *border_color = DARK_GRAY.into();
                style.border = UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(0.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(0.0),
                };
            }
        }
    }
}

#[allow(dead_code)]
pub fn new_color_picked_on_palette_event(
    mut event_reader: EventReader<NewColorPicked>,
    mut node_q: Query<(&mut Children, &mut BackgroundColor), With<ColorPaletteViewTextNode>>,
    mut text_query: Query<&mut Text>,
    mut tool_palette_state_c: ResMut<NextState<ToolPaletteUiState>>,
) {
    for event in event_reader.read() {
        let event_color = event.0;
        for (children, mut bg_color) in node_q.iter_mut() {
            let mut text = text_query.get_mut(children[0]).unwrap();

            //let new_color_hex = convert_color_to_hexstring();
            text.sections[0].value = event_color.to_srgba().to_hex();

            *bg_color = BackgroundColor(event_color);
            tool_palette_state_c.set(ToolPaletteUiState::Pencil);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn trash_palette_button(
    mut trash_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PaletteTrashBtn>),
    >,
    colors: Res<ColorPalette>,
    mut clear_event: EventWriter<ClearSelectionEvent>,
    mut view_event: EventWriter<ViewSelectedTiles>,
) {
    for (interaction, mut color) in &mut trash_query {
        match *interaction {
            Interaction::Pressed => {
                view_event.send(ViewSelectedTiles);
                *color = colors.green_color.into();
                clear_event.send(ClearSelectionEvent);
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.light_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn viewhide_palette_button(
    mut viewhide_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PaletteViewHideBtn>),
    >,
    colors: Res<ColorPalette>,
    mut view: ResMut<ViewablePaletteTiles>,
    mut hide_event: EventWriter<HideSelectedTiles>,
    mut view_event: EventWriter<ViewSelectedTiles>,
) {
    for (interaction, mut color) in &mut viewhide_query {
        match *interaction {
            Interaction::Pressed => {
                if !view.0 {
                    view_event.send(ViewSelectedTiles);
                } else {
                    hide_event.send(HideSelectedTiles);
                }
                view.0 = !view.0;
                *color = colors.green_color.into();
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.light_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn hide_selected_tiles(
    mut event: EventReader<HideSelectedTiles>,
    mut selected_query: Query<&mut Visibility, With<Selected>>,
    mut viewhide_query: Query<&mut UiImage, With<ViewHideImg>>,
    asset_server: Res<AssetServer>,
    mut view: ResMut<ViewablePaletteTiles>,
) {
    for _e in event.read() {
        for mut visi in selected_query.iter_mut() {
            view.0 = false;
            *visi = Visibility::Hidden;
        }
        for mut image in viewhide_query.iter_mut() {
            *image = UiImage::new(asset_server.load("ui/hide_120x120.png"));
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn view_selected_tiles(
    mut event: EventReader<ViewSelectedTiles>,
    mut selected_query: Query<&mut Visibility, With<Selected>>,
    asset_server: Res<AssetServer>,
    mut viewhide_query: Query<&mut UiImage, With<ViewHideImg>>,
    mut view: ResMut<ViewablePaletteTiles>,
) {
    for _e in event.read() {
        for mut visi in selected_query.iter_mut() {
            view.0 = true;
            *visi = Visibility::Visible;
        }
        for mut image in viewhide_query.iter_mut() {
            *image = UiImage::new(asset_server.load("ui/view_120x120.png"));
        }
    }
}
