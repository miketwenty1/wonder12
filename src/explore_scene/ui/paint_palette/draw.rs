use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    building_config::spawn_tile_level,
    componenty::{Land, Location, Selected},
    consty::{MAX_SELECTION_SIZE, TOTAL_TILE_SCALE_SIZE},
    eventy::UpdateUiAmount,
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{SpriteSheetBg, UiInteracting},
};

use super::{
    component::ColorPaletteViewTextNode,
    event::{DrawSelectTileEvent, NewColorPicked},
    state::ToolPaletteUiState,
};

#[allow(clippy::too_many_arguments)]
pub fn mouse_draw_choose_tile(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_camera_simple: Query<(&Camera, &GlobalTransform), With<Camera>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<DrawSelectTileEvent>,
    color_q: Query<&BackgroundColor, With<ColorPaletteViewTextNode>>,
    ui_interacting: Res<UiInteracting>,
) {
    if (mouse.pressed(MouseButton::Left)) && !ui_interacting.0 {
        for (camera, camera_transform) in q_camera_simple.iter_mut() {
            let window = q_window.single();
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                //mycoords.0 = world_position;
                let x = if world_position.x >= 0.0 {
                    ((world_position.x + TOTAL_TILE_SCALE_SIZE / 2. - 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                } else {
                    ((world_position.x - TOTAL_TILE_SCALE_SIZE / 2. + 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                };

                let y = if world_position.y >= 0.0 {
                    ((world_position.y + TOTAL_TILE_SCALE_SIZE / 2. - 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                } else {
                    ((world_position.y - TOTAL_TILE_SCALE_SIZE / 2. + 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                };
                let color = color_q.single().0;
                select_tile_event.send(DrawSelectTileEvent(x, y, color));
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn touch_draw_choose_tile(
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    touches: Res<Touches>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<DrawSelectTileEvent>,
    color_q: Query<&BackgroundColor, With<ColorPaletteViewTextNode>>,
    ui_interacting: Res<UiInteracting>,
) {
    for touch in touches.iter() {
        if touches.get_pressed(touch.id()).is_some() && !ui_interacting.0 {
            for (cam_transform, cam_ortho) in camera.iter_mut() {
                let window = q_window.single();
                let height = window.resolution.height();
                let width = window.resolution.width();

                let world_x = cam_transform.translation.x + touch.position().x * cam_ortho.scale
                    - width / 2. * cam_ortho.scale;
                let world_y = cam_transform.translation.y - touch.position().y * cam_ortho.scale
                    + height / 2. * cam_ortho.scale;

                let x: i32 = if world_x >= 0.0 {
                    ((world_x + TOTAL_TILE_SCALE_SIZE / 2. - 1.) / TOTAL_TILE_SCALE_SIZE) as i32
                } else {
                    ((world_x - TOTAL_TILE_SCALE_SIZE / 2. + 1.) / TOTAL_TILE_SCALE_SIZE) as i32
                };

                let y: i32 = if world_y >= 0.0 {
                    ((world_y + TOTAL_TILE_SCALE_SIZE / 2. - 1.) / TOTAL_TILE_SCALE_SIZE) as i32
                } else {
                    ((world_y - TOTAL_TILE_SCALE_SIZE / 2. + 1.) / TOTAL_TILE_SCALE_SIZE) as i32
                };
                let color = color_q.single().0;
                select_tile_event.send(DrawSelectTileEvent(x, y, color));
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn draw_select_tile(
    mut commands: Commands,
    mut lands: Query<(&mut Location, Entity, &Sprite), With<Land>>,
    texture_atlas: Res<SpriteSheetBg>,
    mut event: EventReader<DrawSelectTileEvent>,
    mut selected_lands: Query<
        (Entity, &mut Sprite, &Location, &Selected),
        (With<Selected>, Without<Land>),
    >,
    //mut last_selected_tile: ResMut<LastSelectedTile>,
    //mut tile_selected_button_q: Query<&mut Visibility, With<SelectedTileUi>>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
    mut toast: EventWriter<ToastEvent>,
    tool_palette_state: Res<State<ToolPaletteUiState>>,
    //mut tool_palette_state_c: ResMut<NextState<ToolPaletteUiState>>,
    mut update_color: EventWriter<NewColorPicked>,
) {
    for e in event.read() {
        let mut done = false;
        //let event_val = ulam::value_of_xy(e.0, e.1);
        for (mut location, parent_entity, underlying_sprite_for_land) in lands.iter_mut() {
            // event location and land query match here
            if location.x == e.0 && location.y == e.1 {
                match **tool_palette_state {
                    ToolPaletteUiState::Pencil => {
                        // looping through all selected tiles
                        for (_ent, mut sprite, slocation, draw_selected) in
                            selected_lands.iter_mut()
                        {
                            // color is new for already selected tile color
                            if slocation.x == e.0
                                && slocation.y == e.1
                                && slocation.x == location.x
                                && slocation.y == location.y
                                && draw_selected.0 != e.2
                            {
                                //info!("despawn branch");
                                // commands.entity(sentity).despawn();
                                // location.selected = false;
                                //change color
                                sprite.color = e.2;
                                done = true;
                            }
                            // nothing to do color is the same
                            if slocation.x == e.0
                                && slocation.y == e.1
                                && slocation.x == location.x
                                && slocation.y == location.y
                                && draw_selected.0 == e.2
                            {
                                //info!("despawn branch");
                                // commands.entity(sentity).despawn();
                                // location.selected = false;
                                //change color
                                done = true;
                            }
                        }

                        if !done
                        // there a new color is being written, overwrite, otherwise nothing
                        {
                            commands
                                .entity(parent_entity)
                                .with_children(|child_builder| {
                                    spawn_tile_level(
                                        101,
                                        &texture_atlas.layout,
                                        &texture_atlas.texture,
                                        child_builder,
                                        e.2,
                                        *location,
                                        Visibility::Inherited,
                                    );
                                });
                            location.selected = true;

                            let count = selected_lands.iter().count();
                            // the "-1" accounts for the one that was just spawned.
                            if count > MAX_SELECTION_SIZE - 1 {
                                toast.send(ToastEvent {
                                    ttype: ToastType::Warn,
                                    message: format!(
                                        "Please unselect some tiles, Maximum {}",
                                        MAX_SELECTION_SIZE
                                    ),
                                });
                            }
                            update_ui_amount_event.send(UpdateUiAmount);
                        }
                    }
                    ToolPaletteUiState::Eraser => {
                        for (ent, _sprite, slocation, _draw_selected) in selected_lands.iter_mut() {
                            // color is new for already selected tile color
                            if slocation.x == e.0
                                && slocation.y == e.1
                                && slocation.x == location.x
                                && slocation.y == location.y
                            {
                                commands.entity(ent).despawn();
                                update_ui_amount_event.send(UpdateUiAmount);
                            }
                        }
                    }
                    ToolPaletteUiState::Eyedrop => {
                        let mut done2 = false;
                        for (_ent, sprite_selected, slocation, draw_selected) in
                            selected_lands.iter_mut()
                        {
                            let a = draw_selected.0;
                            // color is new for already selected tile color
                            if slocation.x == e.0
                                && slocation.y == e.1
                                && slocation.x == location.x
                                && slocation.y == location.y
                            {
                                info!("huh?");
                                update_color.send(NewColorPicked(draw_selected.0));
                                done = true;
                            }
                            info!("huh3?");
                        }
                        if !done2 {
                            info!("huh2?");
                            update_color.send(NewColorPicked(underlying_sprite_for_land.color));
                            done2 = true;
                        }
                    }
                    _ => {
                        info!("I don't believe this should ever be reached for the ToolPaletteUiState match");
                    }
                }
            }
        }
    }
}
