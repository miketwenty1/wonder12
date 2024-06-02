use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    building_config::spawn_tile_level,
    componenty::{BuildingStructure, Land, Location, Selected, SelectedTileUi},
    consty::{MAX_SELECTION_SIZE, TOTAL_TILE_SCALE_SIZE},
    eventy::{SelectTileEvent, UpdateUiAmount},
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{LastSelectedTile, SpriteSheetSelect},
};

#[allow(clippy::too_many_arguments)]
pub fn choose_tile(
    mouse: Res<ButtonInput<MouseButton>>,
    mut q_camera_simple: Query<(&Camera, &GlobalTransform), With<Camera>>,

    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<SelectTileEvent>,
    //mut last_selected_tile: ResMut<LastSelectedTile>,
    //location_query: Query<&Location>,
) {
    if mouse.just_pressed(MouseButton::Left) {
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

                select_tile_event.send(SelectTileEvent(x, y));
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn select_tile(
    mut commands: Commands,
    mut lands: Query<(&mut Location, Entity), (With<Land>, Without<BuildingStructure>)>,
    texture_atlas: Res<SpriteSheetSelect>,
    mut event: EventReader<SelectTileEvent>,
    mut selected_lands: Query<
        (Entity, &Location),
        (With<Selected>, Without<Land>, Without<BuildingStructure>),
    >,
    mut last_selected_tile: ResMut<LastSelectedTile>,
    mut tile_selected_button_q: Query<&mut Visibility, With<SelectedTileUi>>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
    mut toast: EventWriter<ToastEvent>,
) {
    for e in event.read() {
        //let event_val = ulam::value_of_xy(e.0, e.1);
        for (mut location, parent_entity) in lands.iter_mut() {
            if location.x == e.0 && location.y == e.1 {
                if last_selected_tile.0 == e.0 && last_selected_tile.1 == e.1 && !location.selected
                {
                    commands
                        .entity(parent_entity)
                        .with_children(|child_builder| {
                            spawn_tile_level(
                                100,
                                &texture_atlas.layout,
                                &texture_atlas.texture,
                                child_builder,
                                Color::Rgba {
                                    red: 1.,
                                    green: 1.,
                                    blue: 1.,
                                    alpha: 1.,
                                },
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
                            message: "Please unselect some tiles, Maximum 100".to_string(),
                        });
                    }

                    for mut visibility in tile_selected_button_q.iter_mut() {
                        *visibility = Visibility::Visible;
                    }
                } else if last_selected_tile.0 != e.0
                    && last_selected_tile.1 != e.1
                    && !location.selected
                {
                    //info!("last selected res set for {}, {}", e.0, e.1);
                    *last_selected_tile = LastSelectedTile(e.0, e.1);
                } else if location.selected {
                    for (sentity, slocation) in selected_lands.iter_mut() {
                        if slocation.x == e.0
                            && slocation.y == e.1
                            && slocation.x == location.x
                            && slocation.y == location.y
                        {
                            //info!("despawn branch");
                            commands.entity(sentity).despawn();
                            location.selected = false;
                        }
                    }
                } else {
                    *last_selected_tile = LastSelectedTile(e.0, e.1);
                }
            }
        }
        update_ui_amount_event.send(UpdateUiAmount);
    }
}
