use std::collections::HashMap;

use bevy::{
    input::mouse::MouseMotion, math::Vec3A, prelude::*, render::primitives::Aabb,
    text::Text2dBounds,
};
use ulam::Quad;

use super::{
    core_ui::paint_palette::event::ViewSelectedTiles,
    overlay_ui::toast::{ToastEvent, ToastType},
};
use crate::consty::{CHUNK_TILE_SPAN_MULTIPLIER, INDEX_WHITE_LAND, WHITE_COLOR_SRGBA};
use crate::resourcey::{ColorMapToggle, MapTileMode, SpriteSheetLand};
use crate::{building_config::utils::get_text_color, utils::bits_to_target_hash};
use crate::{
    building_config::{spawn_tile_level, utils::sanitize_building_color},
    componenty::{
        AnimationIndices, AnimationTimer, BuildingStructure, BuySelectionButton,
        ClearSelectionButton, InitLoadingNode, InitLoadingText, Land, Location, ManualSelected,
        Selected, SelectedTileUi, TileText, UiNode, UiOverlayingExplorerButton,
    },
    consty::{
        BUILDING_ZOOM_OUT_MAX, CAMERA_SANITY_FACTOR, CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT,
        DESPAWN_TILE_THRESHOLD, MAX_SELECTION_SIZE, TEXT_ZOOM_OUT_MAX, TILE_SCALE,
        TOTAL_TILE_SCALE_SIZE,
    },
    eventy::{
        ClearManualSelectionEvent, ClearSelectionEvent, EdgeEvent, SpriteSpawnEvent,
        UpdateTileTextureEvent, UpdateUiAmount,
    },
    resourcey::{
        ChunkManager, ColorPalette, Edge, InitBlockCount, MaxBlockHeight, SpriteIndexBuilding,
        SpriteSheetBuilding, TileData, ToggleMap, WorldOwnedTileMap,
    },
    statey::{DisplayBuyUiState, InitLoadingBlocksState},
    structy::SpawnDiffData,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// pub fn reset_mouse(
//     mut mouse: ResMut<ButtonInput<MouseButton>>,
//     mut motion: ResMut<Events<MouseMotion>>,
// ) {
//     mouse.clear();
//     mouse.clear_just_pressed(MouseButton::Left);
//     mouse.clear_just_released(MouseButton::Left);
//     motion.clear();
// }

#[allow(clippy::too_many_arguments)]
pub fn init_explorer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    initblocks: Res<InitBlockCount>,
    colors: Res<ColorPalette>,
    mut loading_init_block_text: ResMut<NextState<InitLoadingBlocksState>>,
) {
    info!("initblockcount: {}", initblocks.0);

    commands.spawn((
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
    ));

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
    mut chunk_set: ResMut<ChunkManager>,
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
                chunk_set.set.remove(&ulam_i);
            }
        }
        //debug!("reached edge: {:?}", edge_e.edge_type);
        sprite_spawn_event.send(SpriteSpawnEvent);
        //info!("yo momma1");

        // We are calling so many edge events that this update amount is being called constantly when the camera moves around, just FYI
        update_ui_amount_event.send(UpdateUiAmount);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_block_sprites(
    asset_server: Res<AssetServer>,
    building_texture_mapping: Res<SpriteIndexBuilding>,
    mut sprite_spawn_event: EventReader<SpriteSpawnEvent>,
    mut commands: Commands,
    texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    texture_atlas_handle_land: Res<SpriteSheetLand>,
    edge: Res<Edge>,
    mut chunk_set: ResMut<ChunkManager>,
    tile_map: Res<WorldOwnedTileMap>,
    toggle_map: Res<ToggleMap>,
    max_height: Res<MaxBlockHeight>,
    cam_query: Query<&OrthographicProjection, With<Camera>>,
    map_mode: Res<MapTileMode>,
) {
    for _event in sprite_spawn_event.read() {
        let zoom_level = cam_query.get_single().unwrap().scale;
        // getting whether or not we should spawn text as hidden or visible depending on zoom level
        let text_visibility =
            if *toggle_map.0.get("showtext").unwrap() || zoom_level >= TEXT_ZOOM_OUT_MAX {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
        // getting whether or not we should spawn buildings as hidden or visible depending on zoom level
        let visibility_setting =
            if *toggle_map.0.get("showbuildings").unwrap() || zoom_level >= BUILDING_ZOOM_OUT_MAX {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };

        let middle_y = (edge.top.tile + edge.bottom.tile) / 2;
        let middle_x = (edge.left.tile + edge.right.tile) / 2;

        // removing "4" here as it seem arbitrary. We should make this a CONST
        let spawn_diff = SpawnDiffData {
            xstart: middle_x - CHUNK_TILE_SPAN_COUNT * CHUNK_TILE_SPAN_MULTIPLIER,
            xend: middle_x + CHUNK_TILE_SPAN_COUNT * CHUNK_TILE_SPAN_MULTIPLIER,
            ystart: middle_y - CHUNK_TILE_SPAN_COUNT * CHUNK_TILE_SPAN_MULTIPLIER,
            yend: middle_y + CHUNK_TILE_SPAN_COUNT * CHUNK_TILE_SPAN_MULTIPLIER,
        };

        //info!("spawning {:#?}", spawn_diff);

        // let mut tile_text = "".to_string();

        for x in spawn_diff.xstart..=spawn_diff.xend {
            for y in spawn_diff.ystart..=spawn_diff.yend {
                let building_sprite_index;
                let color_for_sprites;
                let color_for_tile;
                let ulam_i = ulam::value_of_xy(x, y);
                // let mut rng = rand::thread_rng();
                let mut index = 22; //rng.gen_range(0..=INDEX_MAX_LAND);

                if max_height.0 >= ulam_i && !chunk_set.set.contains(&ulam_i) {
                    chunk_set.set.insert(ulam_i);

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

                    let mut value_from_tile = 0;
                    if tile_map.map.contains_key(&locationcoord.ulam) {
                        value_from_tile = tile_map.map.get(&locationcoord.ulam).unwrap().value;
                        building_sprite_index =
                            *building_texture_mapping.0.get(&value_from_tile).unwrap() as usize;
                        color_for_sprites = tile_map.map.get(&locationcoord.ulam).unwrap().color;

                        //Decide what type of tile to show based on the map mode enum

                        (index, color_for_tile) =
                            get_index_color(&map_mode, &tile_map, &locationcoord.ulam);
                    } else {
                        building_sprite_index = 0;
                        color_for_tile = Color::Srgba(Srgba {
                            red: 0.2,
                            green: 0.2,
                            blue: 0.2,
                            alpha: 1.0,
                        });
                        color_for_sprites = color_for_tile;
                    }

                    let mut cmd = commands.spawn((
                        SpriteBundle {
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
                            texture: texture_atlas_handle_land.texture.clone(),
                            ..Default::default()
                        },
                        locationcoord,
                        Land,
                        TextureAtlas {
                            layout: texture_atlas_handle_land.layout.clone(),
                            index,
                        },
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

                    cmd.with_children(|builder| {
                        let slightly_smaller_text_style = TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: get_text_color(&color_for_tile),
                        };

                        let mut text_ent_cmd = builder.spawn((
                            Text2dBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        tile_text,
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

                        text_ent_cmd.insert(Aabb {
                            center: Vec3A::ZERO,
                            half_extents: Vec3A::ZERO,
                        });
                    });

                    let building_color = sanitize_building_color(color_for_sprites.into());

                    cmd.with_children(|builder| {
                        spawn_tile_level(
                            building_sprite_index,
                            &texture_atlas_handle_building.layout,
                            &texture_atlas_handle_building.texture,
                            builder,
                            bevy::prelude::Color::Srgba(building_color),
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
            //edge_type: EdgeType::Left,
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
            //edge_type: EdgeType::Right,
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
            //edge_type: EdgeType::Top,
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
            //edge_type: EdgeType::Bottom,
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
        (&mut TextureAtlas, &mut Sprite, &Location, Entity, &Children),
        (With<Land>, Without<BuildingStructure>),
    >,
    buildings: Query<(&Location, Entity), (Without<Land>, With<BuildingStructure>)>,
    mut event: EventReader<UpdateTileTextureEvent>,
    tile_map: Res<WorldOwnedTileMap>,
    texture_map: Res<SpriteIndexBuilding>,
    texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    toggle_map: Res<ToggleMap>,
    mut text_q: Query<(&mut Text, &Location), With<TileText>>,
    map_mode: Res<MapTileMode>,
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
        //let hiding_colors = toggle_map.0.get("showcolors").unwrap();
        let hiding_buildings = toggle_map.0.get("showbuildings").unwrap();
        let visibility_building_toggle = if *hiding_buildings {
            Visibility::Hidden
        } else {
            Visibility::Visible
        };

        for (mut texture, mut sprite, location, parent_entity, children) in lands.iter_mut() {
            if tile_map.map.contains_key(&location.ulam)
                && tile_map_from_e.contains_key(&location.ulam)
            {
                // making it where the event is driving not the tile resource
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

                (texture.index, sprite.color) =
                    get_index_color(&map_mode, &tile_map, &locationcoord.ulam);
                // if *hiding_colors {
                //     sprite.color = Color::Srgba(Srgba {
                //         red: 1.0,
                //         green: 1.0,
                //         blue: 1.0,
                //         alpha: 1.0,
                //     });
                //     texture.index = tile_map.map.get(&locationcoord.ulam).unwrap().land_index;
                // } else {
                //     sprite.color = tile_data.color;
                //     texture.index = INDEX_WHITE_LAND;
                // }

                // Not really a big deal to do this everytime because likely after each purchase we will need to configure the buildings differently.
                /////////////////////
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
                            bevy::prelude::Color::Srgba(sanitize_building_color(
                                tile_data.color.into(),
                            )),
                            locationcoord,
                            visibility_building_toggle,
                        );
                    });
                /////////////////////
            }
            let (mut text, loc) = text_q.get_mut(children[0]).unwrap();

            //for (mut text, loc) in text_q.iter_mut() {
            if let Some(val) = tile_map.map.get(&loc.ulam) {
                if !*hiding_text {
                    text.sections[0].style.color = get_text_color(&sprite.color);
                    if *showing_value {
                        text.sections[0].value = val.cost.to_string();
                    } else {
                        text.sections[0].value = val.height.to_string();
                    }
                }
            }
            // }
        }

        // for (mut text, loc) in text_q.iter_mut() {
        //     if let Some(val) = tile_map.map.get(&loc.ulam) {
        //         if !*hiding_text {
        //             if *showing_value {
        //                 text.sections[0].value = val.cost.to_string();
        //             } else {
        //                 text.sections[0].value = val.height.to_string();
        //             }
        //         }
        //     }
        // }
    }
}

pub fn get_index_color(
    map_mode: &MapTileMode,
    tile_map: &WorldOwnedTileMap,
    height: &u32,
) -> (usize, Color) {
    match map_mode.0 {
        ColorMapToggle::GameColor => {
            let color = tile_map.map.get(height).cloned().unwrap_or_default().color;
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::LandTile => {
            let index = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .land_index;

            (index, WHITE_COLOR_SRGBA)
        }
        ColorMapToggle::Fee => {
            let fee = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_fee;
            let color = get_fee_color(fee);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::BlockTime => {
            let current = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_time;
            let previous = tile_map
                .map
                .get(&(height - 1))
                .cloned()
                .unwrap_or_default()
                .block_time;

            let color = get_blocktime_color(current - previous);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::TxCount => {
            let txcount = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_n_tx;
            let color = get_tx_count_color(txcount);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::Byte => {
            let bytes = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_size;
            let color = get_byte_color(bytes);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::Weight => {
            let weight = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_weight;
            let color = get_weight_color(weight);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::TargetDifficulty => {
            let bits = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_bits;
            let color = get_bits_color(bits);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::LeadingZeros => {
            let hash = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_hash;
            let leading_zeros = hash.chars().take_while(|&c| c == '0').count();
            let color = get_leading_zeros_color(leading_zeros);

            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::ExcessWork => {
            let hash = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_hash;
            let leading_zeros = hash.chars().take_while(|&c| c == '0').count();
            let bits = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_bits;
            let target_hash = bits_to_target_hash(bits);
            let required_zeros = target_hash.chars().take_while(|&c| c == '0').count();
            //info!("leading: {}, required: {}", leading_zeros, required_zeros);
            let color = get_excesswork_color(leading_zeros - required_zeros);
            (INDEX_WHITE_LAND, color)
        }
        ColorMapToggle::Version => {
            let ver = tile_map
                .map
                .get(height)
                .cloned()
                .unwrap_or_default()
                .block_ver;
            let color = get_version_color(ver);
            (INDEX_WHITE_LAND, color)
        }
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
    mut palette_tiles_view_event: EventWriter<ViewSelectedTiles>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                palette_tiles_view_event.send(ViewSelectedTiles);
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
    mut tile_selected_button_q: Query<&mut Visibility, With<SelectedTileUi>>,
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
        // info!("yo momma2");
        update_ui_amount_event.send(UpdateUiAmount);
    }
}

#[allow(clippy::type_complexity)]
pub fn clear_manual_selection(
    mut selected_q: Query<
        Entity,
        (
            With<Selected>,
            Without<Land>,
            With<ManualSelected>,
            Without<BuildingStructure>,
        ),
    >,
    mut selected_lands_q: Query<&mut Location>,
    mut tile_selected_button_q: Query<&mut Visibility, With<SelectedTileUi>>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
    mut commands: Commands,
    mut clear_event: EventReader<ClearManualSelectionEvent>,
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
        // info!("yo momma3");
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
                        message: format!(
                            "Please unselect some tiles, Maximum {}",
                            MAX_SELECTION_SIZE
                        ),
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

fn get_fee_color(value: i64) -> Color {
    match value {
        // 0: Black
        0 => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 1..=5_000,000: Orange to Red
        1..=5_000_000 => {
            let intensity = value as f32 / 5_000_000.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.5 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 5_000_001..=20_000_000: Red to Pink
        5_000_001..=20_000_000 => {
            let intensity = (value - 5_000_001) as f32 / 15_000_000.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 20_000_001..=100_000_000: Pink to Purpleish Blue
        20_000_001..=100_000_000 => {
            let intensity = (value - 20_000_001) as f32 / 80_000_000.0;
            Color::Srgba(Srgba {
                red: 1.0 - intensity * 0.5,
                green: 0.0,
                blue: 0.5 + intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 100_000_001..=500_000_000: Purple to Magenta
        100_000_001..=500_000_000 => {
            let intensity = (value - 100_000_001) as f32 / 400_000_000.0;
            Color::Srgba(Srgba {
                red: 0.5 + intensity * 0.5,
                green: 0.0,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 500_000_001..=3_000_000_000: Magenta to Hot Pink
        500_000_001..=3_000_000_000 => {
            let intensity = (value - 500_000_001) as f32 / 2_500_000_000.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0 + intensity * 0.5,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 3_000_000_001+: White
        _ => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }),
    }
}

fn get_blocktime_color(value: i64) -> Color {
    match value {
        // -infinity to 0: White
        i64::MIN..=0 => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }),

        // 1 to 600: Light Green to Green
        1..=600 => {
            let intensity = value as f32 / 600.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.8 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 601 to 1800: Light Blue to Blue
        601..=1800 => {
            let intensity = (value - 601) as f32 / 1200.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.5 - intensity * 0.5,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // 1801 to 3600: Yellow to Dark Yellow
        1801..=3600 => {
            let intensity = (value - 1801) as f32 / 1800.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 1.0 - intensity * 0.3,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 3601 to 7200: Light Orange to Orange
        3601..=7200 => {
            let intensity = (value - 3601) as f32 / 3600.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.5 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 7201 to 10800: Light Red to Dark Red
        7201..=10800 => {
            let intensity = (value - 7201) as f32 / 3600.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: intensity * 0.2,
                alpha: 1.0,
            })
        }

        // 10801+: Black
        _ => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),
    }
}

fn get_tx_count_color(value: i32) -> Color {
    match value {
        // 1: Black
        1 => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 2 to 100: Light Green to Green
        2..=100 => {
            let intensity = (value - 2) as f32 / 98.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.8 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 101 to 999: Light Blue to Blue
        101..=999 => {
            let intensity = (value - 101) as f32 / 898.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.5 - intensity * 0.5,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // 1000 to 3000: Yellow to Orange
        1000..=3000 => {
            let intensity = (value - 1000) as f32 / 2000.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 1.0 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 3001 to 6000: Orange to Red
        3001..=6000 => {
            let intensity = (value - 3001) as f32 / 2999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.5 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 6001 to 9000: Red to Pink
        6001..=9000 => {
            let intensity = (value - 6001) as f32 / 2999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 9001+: White
        _ => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }),
    }
}

fn get_byte_color(value: i32) -> Color {
    match value {
        // 0 to 200: Black
        0..=200 => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 201 to 10,000: Light Green to Green
        201..=10_000 => {
            let intensity = (value - 201) as f32 / 9_800.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.8 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 10,001 to 50,000: Light Blue to Blue
        10_001..=50_000 => {
            let intensity = (value - 10_001) as f32 / 39_999.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.5 - intensity * 0.5,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // 50,001 to 200,000: Yellow to Light Orange
        50_001..=200_000 => {
            let intensity = (value - 50_001) as f32 / 149_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 1.0 - intensity * 0.2,
                blue: intensity * 0.2,
                alpha: 1.0,
            })
        }

        // 200,001 to 400,000: Light Orange to Orange
        200_001..=400_000 => {
            let intensity = (value - 200_001) as f32 / 199_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.8 - intensity * 0.3,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 400,001 to 600,000: Light Red to Red
        400_001..=600_000 => {
            let intensity = (value - 400_001) as f32 / 199_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: intensity * 0.5,
                blue: intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 600,001 to 800,000: Light Purple to Magenta
        600_001..=800_000 => {
            let intensity = (value - 600_001) as f32 / 199_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: intensity * 0.0,
                blue: 1.0 - intensity,
                alpha: 1.0,
            })
        }

        // 800,001 to 999,900: Light Magenta to Magenta
        800_001..=999_900 => {
            let intensity = (value - 800_001) as f32 / 199_899.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 999,901+: White
        _ => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }),
    }
}

fn get_weight_color(value: i64) -> Color {
    match value {
        // 0 to 800: Black
        0..=800 => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 801 to 40,000: Light Green to Green
        801..=40_000 => {
            let intensity = (value - 801) as f32 / 39_199.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.8 - intensity * 0.5,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 40,001 to 200,000: Light Blue to Blue
        40_001..=200_000 => {
            let intensity = (value - 40_001) as f32 / 159_999.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.5 - intensity * 0.5,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // 200,001 to 800,000: Yellow to Light Orange
        200_001..=800_000 => {
            let intensity = (value - 200_001) as f32 / 599_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 1.0 - intensity * 0.2,
                blue: intensity * 0.2,
                alpha: 1.0,
            })
        }

        // 800,001 to 1,600,000: Light Orange to Orange
        800_001..=1_600_000 => {
            let intensity = (value - 800_001) as f32 / 799_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.8 - intensity * 0.3,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 1,600,001 to 2,400,000: Light Red to Red
        1_600_001..=2_400_000 => {
            let intensity = (value - 1_600_001) as f32 / 799_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: intensity * 0.5,
                blue: intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 2,400,001 to 3,200,000: Light Purple to Magenta
        2_400_001..=3_200_000 => {
            let intensity = (value - 2_400_001) as f32 / 799_999.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 1.0 - intensity,
                alpha: 1.0,
            })
        }

        // 3,200,001 to 3,999,600: Light Magenta to Magenta
        3_200_001..=3_999_600 => {
            let intensity = (value - 3_200_001) as f32 / 799_599.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 3,999,601+: White
        _ => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }),
    }
}

fn get_bits_color(value: i64) -> Color {
    // Create a hasher and hash the value
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    // Extract RGB values from the hash
    let r = (hash & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = ((hash >> 16) & 0xFF) as f32 / 255.0;

    // Return the color
    Color::Srgba(Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    })
}

pub fn get_leading_zeros_color(value: usize) -> Color {
    match value {
        // 0-7: Black (Very low difficulty)
        0..=7 => Color::Srgba(Srgba {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 8-11: Light Green to Green (Low difficulty)
        8..=11 => {
            let intensity = (value - 8) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.8 - intensity * 0.4,
                blue: 0.0,
                alpha: 1.0,
            })
        }

        // 12-15: Green to Cyan (Moderate difficulty)
        12..=15 => {
            let intensity = (value - 12) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: 0.4 - intensity * 0.2,
                blue: intensity * 0.8,
                alpha: 1.0,
            })
        }

        // 16-19: Cyan to Blue (Increased difficulty)
        16..=19 => {
            let intensity = (value - 16) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: 0.0,
                green: intensity * 0.5,
                blue: 1.0,
                alpha: 1.0,
            })
        }

        // 20-23: Blue to Magenta (High difficulty)
        20..=23 => {
            let intensity = (value - 20) as f32 / 3.0;
            Color::Srgba(Srgba {
                red: intensity * 1.0,
                green: 0.0,
                blue: 1.0 - intensity * 0.5,
                alpha: 1.0,
            })
        }

        // 24+: Magenta to White (Very high difficulty)
        _ => {
            let intensity = (value - 24) as f32 / 40.0;
            Color::Srgba(Srgba {
                red: 1.0,
                green: intensity * 1.0,
                blue: 1.0,
                alpha: 1.0,
            })
        }
    }
}

pub fn get_excesswork_color(value: usize) -> Color {
    match value {
        // 1: Green
        0 => Color::Srgba(Srgba {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 2: Light Blue
        1 => Color::Srgba(Srgba {
            red: 0.5,
            green: 0.5,
            blue: 1.0,
            alpha: 1.0,
        }),

        // 4: Yellow
        2 => Color::Srgba(Srgba {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 5: Orange
        3 => Color::Srgba(Srgba {
            red: 1.0,
            green: 0.5,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 6: Red
        4 => Color::Srgba(Srgba {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }),

        // 7: Purplish Magenta
        5 => Color::Srgba(Srgba {
            red: 0.8,
            green: 0.0,
            blue: 0.8,
            alpha: 1.0,
        }),

        // 6+: Hot Pink (Intensifying with higher values)
        _ => {
            let intensity = ((value - 8) as f32 / 4.0).min(1.0);
            Color::Srgba(Srgba {
                red: 1.0,
                green: 0.0,
                blue: 0.5 + intensity * 0.5,
                alpha: 1.0,
            })
        }
    }
}

fn get_version_color(value: i32) -> Color {
    // Create a hasher and hash the value
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    // Extract RGB values from the hash
    let r = (hash & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = ((hash >> 16) & 0xFF) as f32 / 255.0;

    // Return the color
    Color::Srgba(Srgba {
        red: r,
        green: g,
        blue: b,
        alpha: 1.0,
    })
}
