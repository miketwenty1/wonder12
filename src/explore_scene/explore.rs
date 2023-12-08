use bevy::{input::mouse::MouseMotion, prelude::*, text::Text2dBounds, window::PrimaryWindow};
use rand::Rng;
use ulam::Quad;

use crate::{
    building_config::spawn_tile_level,
    componenty::{
        AnimationIndices, AnimationTimer, BuildingStructure, ClearSelectionButton,
        DetailSelectionButton, Land, Location, Selected, TileText, UiNode, UiTileSelectedButton,
        ZoomInButton, ZoomOutButton,
    },
    consty::{
        CAMERA_SANITY_FACTOR, CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT, DESPAWN_TILE_THRESHOLD,
        HOVERED_BUTTON, MOVE_VELOCITY_FACTOR, NORMAL_BUTTON, PRESSED_BUTTON, TILE_PIXEL_SIZE,
        TILE_SCALE, TOTAL_TILE_SCALE_SIZE,
    },
    eventy::{
        EdgeEvent, SelectTileEvent, SpriteSpawnEvent, UpdateTileTextureEvent, UpdateUiAmount,
    },
    resourcey::{
        ChunkManager, Edge, LastSelectedTile, SpriteIndexBuilding, SpriteSheetBgRes,
        SpriteSheetBuildingRes, TileMap,
    },
    statey::DisplayUiState,
    structy::{EdgeType, SpawnDiffData},
};

pub fn setup_explorer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
) {
    // ui camera

    let texture_handle = asset_server.load("spritesheet/grassdirtbg.png");
    let texture_atlas_bg = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_PIXEL_SIZE, TILE_PIXEL_SIZE),
        12,
        1,
        Some(Vec2::new(0.0, 0.0)),
        None,
    );
    let texture_handle_buildings = asset_server.load("spritesheet/buildings.png");
    let texture_atlas_building = TextureAtlas::from_grid(
        texture_handle_buildings,
        Vec2::new(32.0, 32.0),
        17,
        1,
        Some(Vec2::new(0.0, 0.0)),
        Some(Vec2::new(0.0, 0.0)),
    );
    let texture_atlas_handle_bg = texture_atlases.add(texture_atlas_bg);
    let texture_atlas_handle_building = texture_atlases.add(texture_atlas_building);

    commands.insert_resource(SpriteSheetBgRes(texture_atlas_handle_bg.clone()));
    commands.insert_resource(SpriteSheetBuildingRes(
        texture_atlas_handle_building.clone(),
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            UiNode,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ClearSelectionButton,
                    UiTileSelectedButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Clear",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ZoomOutButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "-",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ZoomInButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "+bo",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(100.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        background_color: NORMAL_BUTTON.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    DetailSelectionButton,
                    UiTileSelectedButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Details",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    sprite_spawn_event.send(SpriteSpawnEvent);
}

#[allow(clippy::too_many_arguments)]
pub fn mouse_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut q_camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    mut q_camera_simple: Query<(&Camera, &GlobalTransform), With<Camera>>,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<SelectTileEvent>,
    //mut last_selected_tile: ResMut<LastSelectedTile>,
    //location_query: Query<&Location>,
) {
    for event in mouse_motion_events.read() {
        if mouse.pressed(MouseButton::Middle) || mouse.pressed(MouseButton::Left) {
            for (mut cam_transform, cam_ortho) in q_camera.iter_mut() {
                let direction = Vec3::new(-event.delta.x, event.delta.y, 0.0);
                cam_transform.translation += direction
                    * time.delta_seconds()
                    * TILE_SCALE
                    * cam_ortho.scale
                    * MOVE_VELOCITY_FACTOR
                    * 1.0;
                set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
            }
        }
    }
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

#[allow(clippy::too_many_arguments)]
pub fn touch_event_system(
    touches: Res<Touches>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<SelectTileEvent>,
    //mut last_selected_tile: ResMut<LastSelectedTile>,
    //location_query: Query<&Location>,
) {
    for touch in touches.iter() {
        for (mut cam_transform, cam_ortho) in camera.iter_mut() {
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

            let direction = Vec3::new(-touch.delta().x, touch.delta().y, 0.0);
            cam_transform.translation +=
                direction * time.delta_seconds() * cam_ortho.scale * MOVE_VELOCITY_FACTOR * 5.0;

            set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);

            if touches.just_pressed(touch.id()) {
                //info!("send touch select");
                select_tile_event.send(SelectTileEvent(x, y));
                //*last_selected_tile = LastSelectedTile(x, y);
            }

            //info!("touch World coords: {}/{}", x, y);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn zoom_out_button_system(
    mut mouse: ResMut<Input<MouseButton>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<ZoomOutButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                text.sections[0].value = "-".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::GRAY;
                for mut ortho in cam_query.iter_mut() {
                    ortho.scale += 0.25;
                    //info!("{}", ortho.scale);
                    if ortho.scale > 5.0 {
                        ortho.scale = 5.0;
                    }
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "-".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "-".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn zoom_in_button_system(
    mut mouse: ResMut<Input<MouseButton>>,
    //mut touch: ResMut<Touches>, // need a clear method or a clear fn work around
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<ZoomInButton>),
    >,
    mut text_query: Query<&mut Text>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                mouse.clear_just_pressed(MouseButton::Left);
                text.sections[0].value = "+".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::GRAY;
                for mut ortho in cam_query.iter_mut() {
                    ortho.scale -= 0.25;
                    if ortho.scale < 0.25 {
                        ortho.scale = 0.25;
                    }
                    //info!("{}", ortho.scale);
                }
            }
            Interaction::Hovered => {
                text.sections[0].value = "+".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "+".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn edge_system(
    mut commands: Commands,
    blocks: Query<(Entity, &Location), With<Land>>,
    mut edge_event: EventReader<EdgeEvent>,
    mut chunk_map: ResMut<ChunkManager>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
) {
    for edge_e in edge_event.read() {
        for (block_entity, block_location) in blocks.iter() {
            if (block_location.y - edge_e.y).abs() > DESPAWN_TILE_THRESHOLD
                || (block_location.x - edge_e.x).abs() > DESPAWN_TILE_THRESHOLD
            {
                //info!("despawning");
                let ulam_i = ulam::value_of_xy(block_location.x, block_location.y);
                commands.entity(block_entity).despawn_recursive();
                chunk_map.map.remove(&ulam_i);
            }
        }

        //debug!("reached edge: {:?}", edge_e.edge_type);

        sprite_spawn_event.send(SpriteSpawnEvent);
        update_ui_amount_event.send(UpdateUiAmount);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_block_sprites(
    asset_server: Res<AssetServer>,
    texture_map: Res<SpriteIndexBuilding>,
    mut sprite_spawn_event: EventReader<SpriteSpawnEvent>,
    mut commands: Commands,
    texture_atlas_handle_bg: Res<SpriteSheetBgRes>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
    //mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    edge: Res<Edge>,
    mut chunk_map: ResMut<ChunkManager>,
    tile_map: Res<TileMap>,
    //toggle_map: Res<ToggleMap>,
) {
    for _event in sprite_spawn_event.read() {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let slightly_smaller_text_style = TextStyle {
            font,
            font_size: 24.0,
            color: Color::WHITE,
        };

        let middle_y = (edge.top.tile + edge.bottom.tile) / 2;
        let middle_x = (edge.left.tile + edge.right.tile) / 2;

        //info!("middle_y: {}, middle_x: {}", middle_y, middle_x);
        let spawn_diff = SpawnDiffData {
            xstart: middle_x - CHUNK_TILE_SPAN_COUNT,
            xend: middle_x + CHUNK_TILE_SPAN_COUNT,
            ystart: middle_y - CHUNK_TILE_SPAN_COUNT,
            yend: middle_y + CHUNK_TILE_SPAN_COUNT,
        };

        //info!("spawning {:#?}", spawn_diff);
        let mut building_sprite_index;
        let mut land_sprite_index: usize;
        let mut color_for_sprites;
        let mut color_for_tile;
        // let mut tile_text = "".to_string();

        for x in spawn_diff.xstart..=spawn_diff.xend {
            for y in spawn_diff.ystart..=spawn_diff.yend {
                let ulam_i = ulam::value_of_xy(x, y);
                if !chunk_map.map.contains_key(&ulam_i) {
                    chunk_map.map.insert(ulam_i, true);

                    //info!("spawning: x: {}, y: {}", x, y);

                    let mut locationcoord = Location {
                        x,
                        y,
                        ulam: ulam::value_of_xy(x, y),
                        quad: ulam::quad_of_xy(x, y),
                        selected: false,
                    };
                    if locationcoord.ulam == 1 {
                        locationcoord.quad = Quad::SouthEast
                    } else if locationcoord.quad == Quad::SouthEast {
                        locationcoord.quad = Quad::South
                    } else if locationcoord.quad == Quad::East
                        && ulam::quad_of_value(locationcoord.ulam - 1) == Quad::SouthEast
                    {
                        locationcoord.quad = Quad::SouthEast;
                    }

                    // writing this code to make tile_text populate correctly where it updates tiles correctly based on toggle.

                    let mut rng = rand::thread_rng();
                    land_sprite_index = rng.gen_range(1..=11);

                    if tile_map.map.contains_key(&locationcoord.ulam) {
                        let amount_from_tile =
                            tile_map.map.get(&locationcoord.ulam).unwrap().amount;
                        building_sprite_index =
                            *texture_map.0.get(&amount_from_tile).unwrap() as usize;

                        color_for_sprites = tile_map.map.get(&locationcoord.ulam).unwrap().color;
                        land_sprite_index =
                            tile_map.map.get(&locationcoord.ulam).unwrap().land_index;
                        color_for_tile = Color::Rgba {
                            red: 1.,
                            green: 1.,
                            blue: 1.,
                            alpha: 1.,
                        };
                    } else {
                        building_sprite_index = 0;
                        color_for_tile = Color::Rgba {
                            red: 0.2,
                            green: 0.2,
                            blue: 0.2,
                            alpha: 1.0,
                        };
                        color_for_sprites = color_for_tile;
                    }
                    commands
                        .spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle_bg.0.clone(), //textureatlashandle.clone(),
                                sprite: TextureAtlasSprite {
                                    color: color_for_tile,
                                    index: land_sprite_index,
                                    ..Default::default()
                                },

                                transform: Transform {
                                    translation: Vec3::new(
                                        TOTAL_TILE_SCALE_SIZE * x as f32,
                                        TOTAL_TILE_SCALE_SIZE * y as f32,
                                        0.,
                                    ),
                                    scale: Vec3::new(TILE_SCALE, TILE_SCALE, 1.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            locationcoord,
                            Land,
                        ))
                        .with_children(|builder| {
                            builder.spawn((
                                Text2dBundle {
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            format!("{}", locationcoord.ulam),
                                            slightly_smaller_text_style.clone(),
                                        )],
                                        alignment: TextAlignment::Left,
                                        ..Default::default()
                                    },
                                    text_2d_bounds: Text2dBounds { ..default() },
                                    transform: Transform {
                                        translation: Vec3::new(0., 0., 3.),
                                        scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                                        ..Default::default()
                                    },
                                    ..default()
                                },
                                locationcoord,
                                TileText,
                            ));
                        })
                        .with_children(|builder| {
                            spawn_tile_level(
                                building_sprite_index,
                                &texture_atlas_handle_building.0.clone(),
                                builder,
                                color_for_sprites,
                                locationcoord,
                                Visibility::Visible,
                            );
                        });
                }
            }
        }
    }
}

pub fn set_camera_tile_bounds(
    mut camera_vec3: Vec3,
    edge: &mut ResMut<Edge>,
    edge_event: &mut EventWriter<EdgeEvent>,
) {
    if camera_vec3.x < edge.left.pixel {
        edge.left.pixel -= CHUNK_PIXEL_SIZE;
        edge.left.tile -= CHUNK_TILE_SPAN_COUNT;
        edge.right.pixel -= CHUNK_PIXEL_SIZE;
        edge.right.tile -= CHUNK_TILE_SPAN_COUNT;

        edge_event.send(EdgeEvent {
            edge_type: EdgeType::Left,
            x: edge.left.tile,
            y: (edge.top.tile + edge.bottom.tile) / 2,
        });
    }
    if camera_vec3.x > edge.right.pixel {
        //cam_transform.translation.x = edge.right.pixel;
        edge.right.pixel += CHUNK_PIXEL_SIZE;
        edge.right.tile += CHUNK_TILE_SPAN_COUNT;
        edge.left.pixel += CHUNK_PIXEL_SIZE;
        edge.left.tile += CHUNK_TILE_SPAN_COUNT;
        edge_event.send(EdgeEvent {
            edge_type: EdgeType::Right,
            x: edge.right.tile,
            y: (edge.top.tile + edge.bottom.tile) / 2,
        });
        //info!("new right {}", edge.right.pixel);

        if camera_vec3.x > edge.right.pixel * CAMERA_SANITY_FACTOR {
            //info!("adjust right?");
            camera_vec3.x = edge.right.pixel;
        }
    }
    if camera_vec3.y > edge.top.pixel {
        //cam_transform.translation.y = edge.top.pixel;
        edge.top.pixel += CHUNK_PIXEL_SIZE;
        edge.top.tile += CHUNK_TILE_SPAN_COUNT;
        edge.bottom.pixel += CHUNK_PIXEL_SIZE;
        edge.bottom.tile += CHUNK_TILE_SPAN_COUNT;
        edge_event.send(EdgeEvent {
            edge_type: EdgeType::Top,
            x: (edge.left.tile + edge.right.tile) / 2,
            y: edge.top.tile,
        });

        //info!("new top {}", edge.top.pixel);
        if camera_vec3.y > edge.top.pixel * CAMERA_SANITY_FACTOR {
            //info!("adjust top");
            camera_vec3.y = edge.top.pixel;
        }
    }
    if camera_vec3.y < edge.bottom.pixel {
        //cam_transform.translation.y = edge.bottom.pixel;
        edge.bottom.pixel -= CHUNK_PIXEL_SIZE;
        edge.bottom.tile -= CHUNK_TILE_SPAN_COUNT;
        edge.top.pixel -= CHUNK_PIXEL_SIZE;
        edge.top.tile -= CHUNK_TILE_SPAN_COUNT;
        edge_event.send(EdgeEvent {
            edge_type: EdgeType::Bottom,
            x: (edge.left.tile + edge.right.tile) / 2,
            y: edge.bottom.tile,
        });
        //info!("new bottom {}", edge.bottom.pixel);
        if camera_vec3.y < edge.bottom.pixel * CAMERA_SANITY_FACTOR {
            //info!("adjust bottom");
            camera_vec3.y = edge.bottom.pixel;
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn update_tile_textures(
    mut commands: Commands,
    mut lands: Query<
        (&mut TextureAtlasSprite, &Location, Entity),
        (With<Land>, Without<BuildingStructure>),
    >,
    mut event: EventReader<UpdateTileTextureEvent>,
    tile_map: Res<TileMap>,
    texture_map: Res<SpriteIndexBuilding>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
    //toggle_map: Res<ToggleMap>,
) {
    for _e in event.read() {
        // let dd = toggle_map.0.get("showbuildings").unwrap();
        // let mut visibility_building_toggle;
        // if !*dd {
        //     visibility_building_toggle = Visibility::Visible;
        // } else {
        //     visibility_building_toggle = Visibility::Hidden;
        // }
        for (mut texture, location, parent_entity) in lands.iter_mut() {
            if tile_map.map.contains_key(&location.ulam) {
                let tile_data = tile_map.map.get(&location.ulam).unwrap();
                let building_sprite_index = *texture_map.0.get(&tile_data.amount).unwrap() as usize;

                let c = ulam::calc_coord::calc_coord(tile_data.height);
                let mut locationcoord = Location {
                    x: c.x,
                    y: c.y,
                    ulam: tile_data.height,
                    quad: ulam::quad_of_xy(c.x, c.y),
                    selected: false,
                };
                if locationcoord.ulam == 1 {
                    locationcoord.quad = Quad::SouthEast
                } else if locationcoord.quad == Quad::SouthEast {
                    locationcoord.quad = Quad::South
                } else if locationcoord.quad == Quad::East
                    && ulam::quad_of_value(locationcoord.ulam - 1) == Quad::SouthEast
                {
                    locationcoord.quad = Quad::SouthEast;
                }

                texture.color = Color::Rgba {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                    alpha: 1.0,
                };
                //let base_sprite_index: usize = rng.gen_range(1..=11);
                let land_sprite_index = tile_map.map.get(&locationcoord.ulam).unwrap().land_index;
                texture.index = land_sprite_index; //*texture_map.0.get(&base_sprite_index).unwrap() as usize;

                commands
                    .entity(parent_entity)
                    .with_children(|child_builder| {
                        spawn_tile_level(
                            building_sprite_index,
                            &texture_atlas_handle_building.0.clone(),
                            child_builder,
                            tile_data.color,
                            locationcoord,
                            Visibility::Visible, //visibility_building_toggle,
                        );
                    });
            }
        }
        //info!("updated textures");
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn select_tile(
    mut commands: Commands,
    mut lands: Query<(&mut Location, Entity), (With<Land>, Without<BuildingStructure>)>,
    //chunk_map: Res<ChunkManager>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
    mut event: EventReader<SelectTileEvent>,
    mut selected_lands: Query<
        (Entity, &Location),
        (With<Selected>, Without<Land>, Without<BuildingStructure>),
    >,
    mut last_selected_tile: ResMut<LastSelectedTile>,
    mut tile_selected_button_q: Query<&mut Visibility, With<UiTileSelectedButton>>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
) {
    for e in event.read() {
        //let event_val = ulam::value_of_xy(e.0, e.1);
        for (mut location, parent_entity) in lands.iter_mut() {
            if location.x == e.0 && location.y == e.1 {
                if last_selected_tile.0 == e.0 && last_selected_tile.1 == e.1 && !location.selected
                {
                    // spawn
                    //info!("spawn branch1");
                    commands
                        .entity(parent_entity)
                        .with_children(|child_builder| {
                            spawn_tile_level(
                                100,
                                &texture_atlas_handle_building.0.clone(),
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
                    update_ui_amount_event.send(UpdateUiAmount);

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
                            update_ui_amount_event.send(UpdateUiAmount);
                        }
                    }
                    if selected_lands.iter_mut().len() <= 1 {
                        for mut visibility in tile_selected_button_q.iter_mut() {
                            *visibility = Visibility::Hidden;
                        }
                    };
                } else {
                    *last_selected_tile = LastSelectedTile(e.0, e.1);
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn clear_selection_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            With<ClearSelectionButton>,
        ),
    >,
    mut commands: Commands,
    mut text_query: Query<&mut Text>,
    mut selected_q: Query<Entity, (With<Selected>, Without<Land>, Without<BuildingStructure>)>,
    mut selected_lands_q: Query<&mut Location>,
    mut tile_selected_button_q: Query<&mut Visibility, With<UiTileSelectedButton>>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Clear".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::GRAY;
                for sentity in selected_q.iter_mut() {
                    commands.entity(sentity).despawn();
                }
                for mut location in selected_lands_q.iter_mut() {
                    location.selected = false;
                }
                for mut visibility in tile_selected_button_q.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
                update_ui_amount_event.send(UpdateUiAmount);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Clear".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Clear".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn detail_selection_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (
            Changed<Interaction>,
            With<Button>,
            With<DetailSelectionButton>,
        ),
    >,
    //    mut commands: Commands,
    mut text_query: Query<&mut Text>,
    mut ui_state: ResMut<NextState<DisplayUiState>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Details".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::GRAY;
                ui_state.set(DisplayUiState::On);
                info!("coming soon");
            }
            Interaction::Hovered => {
                text.sections[0].value = "Details".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                text.sections[0].value = "Details".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
