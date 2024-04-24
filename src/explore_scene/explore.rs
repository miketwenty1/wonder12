use std::collections::HashMap;

use bevy::{input::mouse::MouseMotion, prelude::*, text::Text2dBounds};
use rand::Rng;
use ulam::Quad;

use crate::{
    building_config::{spawn_tile_level, utils::sanitize_building_color},
    componenty::{
        AnimationIndices, AnimationTimer, BuildingStructure, BuySelectionButton,
        ClearSelectionButton, InitLoadingNode, InitLoadingText, Land, Location, Selected, TileText,
        UiNode, UiOverlayingExplorerButton, UiTileSelectedButton, ZoomInButton, ZoomOutButton,
    },
    consty::{
        CAMERA_SANITY_FACTOR, CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT, DESPAWN_TILE_THRESHOLD,
        MAX_SELECTION_SIZE, TEXT_ZOOM_OUT_MAX, TILE_SCALE, TOTAL_TILE_SCALE_SIZE,
    },
    eventy::{
        ClearSelectionEvent, EdgeEvent, SpriteSpawnEvent, UpdateTileTextureEvent, UpdateUiAmount,
    },
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{
        ChunkManager, ColorPalette, Edge, InitBlockCount, MaxBlockHeight, SpriteIndexBuilding,
        SpriteSheetBg, SpriteSheetBuilding, TileData, ToggleMap, WorldOwnedTileMap,
    },
    statey::{DisplayBuyUiState, InitLoadingBlocksState},
    structy::{EdgeType, SpawnDiffData},
};

// async fn get_game_data() -> Result<String, JsValue> {
//     let promise = retrieveLocalBrowserGameData();
//     let result = JsFuture::from(promise).await?;
//     Ok(result.as_string().unwrap_or_default())
// }

pub fn reset_mouse(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut motion: ResMut<Events<MouseMotion>>,
) {
    mouse.clear();
    mouse.clear_just_pressed(MouseButton::Left);
    mouse.clear_just_released(MouseButton::Left);
    motion.clear();
}

pub fn init_explorer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    initblocks: Res<InitBlockCount>,
    colors: Res<ColorPalette>,
    mut loading_init_block_text: ResMut<NextState<InitLoadingBlocksState>>,
) {
    info!("initblockcount: {}", initblocks.0);

    // let texture_atlas_handle_bg = texture_atlases.add(texture_atlas_bg);
    // let texture_atlas_handle_building = texture_atlases.add(texture_atlas_building);

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
                        border_color: BorderColor(colors.button_color),
                        background_color: colors.button_color.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    ClearSelectionButton,
                    UiTileSelectedButton,
                    UiOverlayingExplorerButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Clear",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: colors.text_color,
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
                        border_color: BorderColor(colors.node_color),
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
                            font_size: 40.0,
                            color: colors.text_color,
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
                        border_color: BorderColor(colors.node_color),
                        background_color: colors.button_color.into(),
                        ..default()
                    },
                    ZoomInButton,
                    UiOverlayingExplorerButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "+bo",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: colors.text_color,
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
                        background_color: colors.button_color.into(),
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    UiOverlayingExplorerButton,
                    BuySelectionButton,
                    UiTileSelectedButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Buy",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: colors.text_color,
                        },
                    ));
                });
        });

    // this is the same text as below but outlined
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
                ..default()
            },
            InitLoadingNode,
        ))
        .with_children(|child| {
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center, //nope left right
                        justify_items: JustifyItems::Center,
                        margin: UiRect::top(Val::Percent(29.9)),
                        ..default()
                    },
                    // background_color: Color::PINK.into(),
                    ..default()
                })
                .with_children(|childtext| {
                    childtext.spawn((
                        TextBundle::from_section(
                            "Initilizing Game Map 0%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.2,
                                color: colors.text_color,
                            },
                        ),
                        InitLoadingText,
                    ));
                });
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
                ..default()
            },
            InitLoadingNode,
        ))
        .with_children(|child| {
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Start,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center, //nope left right
                        justify_items: JustifyItems::Center,
                        margin: UiRect::top(Val::Percent(30.0)),
                        ..default()
                    },
                    // background_color: Color::PINK.into(),
                    ..default()
                })
                .with_children(|childtext| {
                    childtext.spawn((
                        TextBundle::from_section(
                            "Initilizing Game Map 0%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: colors.accent_color,
                            },
                        ),
                        InitLoadingText,
                    ));
                });
        });

    sprite_spawn_event.send(SpriteSpawnEvent);
    loading_init_block_text.set(InitLoadingBlocksState::On);
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn edge_system(
    mut commands: Commands,
    blocks: Query<(Entity, &Location), With<Land>>,
    mut edge_event: EventReader<EdgeEvent>,
    mut chunk_map: ResMut<ChunkManager>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
) {
    for edge_e in edge_event.read() {
        info!("EDGE EVENT");
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
    texture_atlas_handle_bg: Res<SpriteSheetBg>,
    texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    edge: Res<Edge>,
    mut chunk_map: ResMut<ChunkManager>,
    tile_map: Res<WorldOwnedTileMap>,
    toggle_map: Res<ToggleMap>,
    max_height: Res<MaxBlockHeight>,
    cam_query: Query<&OrthographicProjection, With<Camera>>,
) {
    for _event in sprite_spawn_event.read() {
        let font = asset_server.load("fonts/FiraSans-Bold.ttf");
        let slightly_smaller_text_style = TextStyle {
            font,
            font_size: 24.0,
            color: Color::WHITE,
        };

        let zoom_level = cam_query.get_single().unwrap().scale;
        let text_visibility =
            if *toggle_map.0.get("showtext").unwrap() || zoom_level >= TEXT_ZOOM_OUT_MAX {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };

        let middle_y = (edge.top.tile + edge.bottom.tile) / 2;
        let middle_x = (edge.left.tile + edge.right.tile) / 2;
        //info!("middle x: {}, middle y: {}", middle_x, middle_y);
        let spawn_diff = SpawnDiffData {
            xstart: middle_x - CHUNK_TILE_SPAN_COUNT * 4,
            xend: middle_x + CHUNK_TILE_SPAN_COUNT * 4,
            ystart: middle_y - CHUNK_TILE_SPAN_COUNT * 4,
            yend: middle_y + CHUNK_TILE_SPAN_COUNT * 4,
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

                if max_height.0 >= ulam_i && !chunk_map.map.contains_key(&ulam_i) {
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
                    //land_sprite_index = rng.gen_range(1..=11);
                    let mut value_from_tile = 0;
                    if tile_map.map.contains_key(&locationcoord.ulam) {
                        value_from_tile = tile_map.map.get(&locationcoord.ulam).unwrap().value;
                        building_sprite_index =
                            *texture_map.0.get(&value_from_tile).unwrap() as usize;

                        color_for_sprites = tile_map.map.get(&locationcoord.ulam).unwrap().color;
                        land_sprite_index =
                            tile_map.map.get(&locationcoord.ulam).unwrap().land_index;
                        color_for_tile = Color::Rgba {
                            red: 1.,
                            green: 1.,
                            blue: 1.,
                            alpha: 1.,
                        };
                        if !*toggle_map.0.get("showcolors").unwrap() {
                            land_sprite_index = 0;
                            color_for_tile = color_for_sprites;
                        };
                    } else {
                        land_sprite_index = rng.gen_range(1..=11);
                        building_sprite_index = 0;
                        color_for_tile = Color::Rgba {
                            red: 0.2,
                            green: 0.2,
                            blue: 0.2,
                            alpha: 1.0,
                        };
                        color_for_sprites = color_for_tile;
                    }

                    let mut cmd = commands.spawn((
                        SpriteSheetBundle {
                            atlas: TextureAtlas {
                                layout: texture_atlas_handle_bg.layout.clone(),
                                index: land_sprite_index,
                            },
                            sprite: Sprite {
                                color: color_for_tile,
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
                            texture: texture_atlas_handle_bg.texture.clone(),
                            ..Default::default()
                        },
                        locationcoord,
                        Land,
                    ));

                    // SPAWN correct text for tile based on toggle

                    let tile_text = if *toggle_map.0.get("showvalues").unwrap() {
                        locationcoord.ulam.to_string()
                    } else if *toggle_map.0.get("showheights").unwrap() {
                        let a = value_from_tile;
                        if a == 0 {
                            "".to_string()
                        } else {
                            a.to_string()
                        }
                    } else {
                        "somethingwrongvalue".to_string()
                    };

                    // SPAWN building visibility based on toggle
                    let visibility_setting = if *toggle_map.0.get("showbuildings").unwrap() {
                        Visibility::Hidden
                    } else {
                        Visibility::Visible
                    };

                    cmd.with_children(|builder| {
                        builder.spawn((
                            Text2dBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        tile_text,
                                        //format!("{}", locationcoord.ulam),
                                        slightly_smaller_text_style.clone(),
                                    )],
                                    justify: JustifyText::Left,
                                    ..Default::default()
                                },
                                text_2d_bounds: Text2dBounds { ..default() },
                                transform: Transform {
                                    translation: Vec3::new(0., 0., 5.),
                                    scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                                    ..Default::default()
                                },
                                visibility: text_visibility,
                                ..default()
                            },
                            locationcoord,
                            TileText,
                        ));
                    });

                    let building_color = sanitize_building_color(color_for_sprites);

                    cmd.with_children(|builder| {
                        spawn_tile_level(
                            building_sprite_index,
                            &texture_atlas_handle_building.layout,
                            &texture_atlas_handle_building.texture,
                            builder,
                            building_color,
                            locationcoord,
                            visibility_setting,
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
        //info!("LEFT WRITER");
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
        //info!("RIGHT WRITER");
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
        //info!("TOP WRITER");
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
        //info!("BOTTOM WRITER");
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

// this function is weird because the event takes in UpdateTileTextureEvent but then only
// uses it to check to see if the height is in the tilemap.
// tilemap seems to be soruce of truth for what gets updated with this function
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn update_tile_textures(
    mut commands: Commands,
    mut lands: Query<
        (&mut TextureAtlas, &mut Sprite, &Location, Entity),
        (With<Land>, Without<BuildingStructure>),
    >,
    buildings: Query<(&Location, Entity), (Without<Land>, With<BuildingStructure>)>,
    mut event: EventReader<UpdateTileTextureEvent>,
    tile_map: Res<WorldOwnedTileMap>,
    texture_map: Res<SpriteIndexBuilding>,
    texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    toggle_map: Res<ToggleMap>,
    mut text_q: Query<(&mut Text, &Location), With<TileText>>,
    // mut toggle_buildings: EventWriter<ToggleBuildings>,
    // mut toggle_colors: EventWriter<ToggleColors>,
    // mut toggle_text: EventWriter<ToggleText>,
) {
    for tile_vec in event.read() {
        info!("receving update texture event");

        let tiles = tile_vec.0.clone();
        let tile_map_from_e: HashMap<u32, TileData> =
            tiles.into_iter().map(|tile| (tile.height, tile)).collect();

        // let showing_colors = toggle_map.0.get("hidecolors").unwrap();
        // let showing_buildings = toggle_map.0.get("hidebuildings").unwrap();
        let showing_value = toggle_map.0.get("showheights").unwrap();
        let hiding_text = toggle_map.0.get("showtext").unwrap();
        let hiding_colors = toggle_map.0.get("showcolors").unwrap();
        let hiding_buildings = toggle_map.0.get("showbuildings").unwrap();
        let visibility_building_toggle = if *hiding_buildings {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };
        // info!("for loop");
        // info!("EVENT MAP {:#?}", tile_map_from_e);
        // info!("TILEMAP {:#?}", tile_map.map);
        for (mut texture, mut sprite, location, parent_entity) in lands.iter_mut() {
            if tile_map.map.contains_key(&location.ulam)
                && tile_map_from_e.contains_key(&location.ulam)
            {
                //info!("location.ulam: {}", location.ulam);
                //let tile_data = tile_map.map.get(&location.ulam).unwrap(); // making it where the event is driving not the tile resource
                let tile_data = tile_map_from_e.get(&location.ulam).unwrap();
                // info!("{:#?}", tile_data);
                let building_sprite_index = *texture_map.0.get(&tile_data.value).unwrap() as usize;

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

                // show correct color based on toggle
                if *hiding_colors {
                    sprite.color = Color::Rgba {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                        alpha: 1.0,
                    };
                } else {
                    sprite.color = tile_data.color;
                    texture.index = 0;
                }

                //let base_sprite_index: usize = rng.gen_range(1..=11);
                //let land_sprite_index = tile_map.map.get(&locationcoord.ulam).unwrap().land_index;
                //texture.index = land_sprite_index; //*texture_map.0.get(&base_sprite_index).unwrap() as usize;

                // if (there is some change) {

                // }
                //if building
                for (building_location, building_entity) in buildings.iter() {
                    if building_location.ulam == location.ulam {
                        //info!("despawning old building stuff");
                        commands.entity(building_entity).despawn();
                    }
                }

                commands
                    .entity(parent_entity)
                    .with_children(|child_builder| {
                        //info!("spawning??!");
                        spawn_tile_level(
                            building_sprite_index,
                            &texture_atlas_handle_building.layout,
                            &texture_atlas_handle_building.texture,
                            child_builder,
                            sanitize_building_color(tile_data.color),
                            locationcoord,
                            visibility_building_toggle,
                        );
                    });

                //info!("{:#?}", locationcoord);
            }
        }

        info!("updated textures");
        for (mut text, loc) in text_q.iter_mut() {
            //let a = tile_map.map.get(&loc.ulam);

            // match a {
            //     Some(val) => {
            //         if *hiding_text {
            //         } else if *showing_value {
            //             text.sections[0].value = val.cost.to_string();
            //         } else {
            //             text.sections[0].value = val.height.to_string();
            //         }
            //     }
            //     None => {}
            // }
            if let Some(val) = tile_map.map.get(&loc.ulam) {
                if !*hiding_text {
                    if *showing_value {
                        text.sections[0].value = val.cost.to_string();
                    } else {
                        text.sections[0].value = val.height.to_string();
                    }
                }
            }
        }

        info!("updated text");
        // toggle_buildings.send(ToggleBuildings);
        // toggle_colors.send(ToggleColors);
        // if !*hiding_text {
        //     if *showing_value {
        //         toggle_text.send(ToggleText(TileTextType::Value));
        //     } else {
        //         toggle_text.send(ToggleText(TileTextType::Height));
        //     }
        // } else {
        //     toggle_text.send(ToggleText(TileTextType::Blank));
        // }
    }
}

pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
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
    mut text_query: Query<&mut Text>,
    mut clear_event: EventWriter<ClearSelectionEvent>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "Clear".to_string();
                *color = colors.light_color.into();
                border_color.0 = colors.light_color;
                clear_event.send(ClearSelectionEvent);
            }
            Interaction::Hovered => {
                text.sections[0].value = "Clear".to_string();
                *color = colors.accent_color.into();
                border_color.0 = colors.node_color;
            }
            Interaction::None => {
                text.sections[0].value = "Clear".to_string();
                *color = colors.red_color.into();
                border_color.0 = colors.node_color;
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn clear_selection(
    mut selected_q: Query<Entity, (With<Selected>, Without<Land>, Without<BuildingStructure>)>,
    mut selected_lands_q: Query<&mut Location>,
    mut tile_selected_button_q: Query<&mut Visibility, With<UiTileSelectedButton>>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
    mut commands: Commands,
    mut clear_event: EventReader<ClearSelectionEvent>,
) {
    for _e in clear_event.read() {
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
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn buy_selection_button(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<BuySelectionButton>),
    >,
    //    mut commands: Commands,
    mut text_query: Query<&mut Text>,
    mut ui_state: ResMut<NextState<DisplayBuyUiState>>,
    colors: Res<ColorPalette>,
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touch: EventReader<TouchInput>,
    mut touches: ResMut<Touches>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    selection: Query<&Selected>,
    mut toast: EventWriter<ToastEvent>,
    mut ui_buttons: Query<&mut Visibility, With<UiOverlayingExplorerButton>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = "Buy".to_string();
                *color = colors.green_color.into();
                border_color.0 = colors.light_color;
                // trying to kill jumpiness
                mouse.clear();
                mouse_motion_events.clear();
                touch.clear();
                touches.clear();

                let count = selection.iter().count();
                if count > MAX_SELECTION_SIZE {
                    toast.send(ToastEvent {
                        ttype: ToastType::Bad,
                        message: "Please unselect some tiles, Maximum 100".to_string(),
                    });
                } else {
                    for mut button in ui_buttons.iter_mut() {
                        *button = Visibility::Hidden;
                    }
                    ui_state.set(DisplayBuyUiState::BlockDetail);
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = "Buy".to_string();
                *color = colors.accent_color.into();
                border_color.0 = colors.node_color;
                // mouse.clear();
                // mouse.clear_just_pressed(MouseButton::Left);
                // mouse.clear_just_released(MouseButton::Left);
                // mouse_motion_events.clear();
                // mouse_motion_events.read();
            }
            Interaction::None => {
                //text.sections[0].value = "Buy".to_string();
                *color = colors.green_color.into();
                border_color.0 = colors.node_color;
            }
        }
    }
}
