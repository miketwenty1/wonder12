use std::collections::HashMap;

use bevy::{
    input::mouse::MouseMotion, math::Vec3A, prelude::*, reflect::Enum, render::primitives::Aabb,
    text::FontSmoothing,
};
use ulam::Quad;

use super::{
    blockchain_color::get_index_color,
    core_ui::paint_palette::event::ViewSelectedTiles,
    overlay_ui::toast::{ToastEvent, ToastType},
};
use crate::consty::CHUNK_TILE_SPAN_MULTIPLIER;
use crate::resourcey::{MapTileMode, SpriteSheetLand};
use crate::{building_config::utils::get_text_color, resourcey::BlockExplorerCount};
use crate::{
    building_config::{spawn_tile_level, utils::sanitize_building_color},
    componenty::{
        AnimationIndices, AnimationTimer, BuildingStructure, BuySelectionButton,
        ClearSelectionButton, InitLoadingNode, InitLoadingText, Land, Location, ManualSelected,
        Selected, SelectedTileUi, TileText, UiNode, UiOverlayingExplorerButton,
    },
    consty::{
        BUILDING_ZOOM_OUT_MAX, CAMERA_SANITY_FACTOR, CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT,
        DESPAWN_TILE_THRESHOLD, MAX_SELECTION_SIZE, SCALE_FACTOR, TEXT_ZOOM_OUT_MAX,
        TOTAL_TILE_SCALE_SIZE,
    },
    eventy::{
        ClearManualSelectionEvent, ClearSelectionEvent, EdgeEvent, SpriteSpawnEvent,
        UpdateTileTextureEvent, UpdateUiAmount,
    },
    resourcey::{
        ChunkManager, ColorPalette, Edge, LocalBrowserStorageCount, MaxBlockHeight,
        SpriteIndexBuilding, SpriteSheetBuilding, TileData, ToggleMap, WorldOwnedTileMap,
    },
    statey::{DisplayBuyUiState, InitLoadingBlocksState},
    structy::SpawnDiffData,
};

#[allow(clippy::too_many_arguments)]
pub fn init_explorer(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    initblocks: Res<LocalBrowserStorageCount>,
    colors: Res<ColorPalette>,
    loading_init_block_text: ResMut<NextState<InitLoadingBlocksState>>,
    block_explorer_count: Res<BlockExplorerCount>,
    //local_browser_storage_count: Res<BlockExplorerCount>,
) {
    info!("initblockcount: {}", initblocks.0);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::Center,
            ..default()
        },
        UiNode,
    ));

    // this is the same text as below but outlined
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
            InitLoadingNode,
        ))
        .with_children(|child| {
            child
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center, //nope left right
                    justify_items: JustifyItems::Center,
                    margin: UiRect::top(Val::Percent(29.9)),
                    ..default()
                })
                .with_children(|childtext| {
                    childtext.spawn((
                        Text::new("Initilizing Game Map 0%"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.2,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(colors.text_color),
                        InitLoadingText,
                    ));
                });
        });

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                justify_items: JustifyItems::Center,
                ..default()
            },
            InitLoadingNode,
        ))
        .with_children(|child| {
            child
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center, //nope left right
                    justify_items: JustifyItems::Center,
                    margin: UiRect::top(Val::Percent(30.0)),
                    ..default()
                })
                .with_children(|childtext| {
                    childtext.spawn((
                        Text::new("Initilizing Game Map 0%"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(colors.accent_color),
                        InitLoadingText,
                    ));
                });
        });

    sprite_spawn_event.send(SpriteSpawnEvent);

    // if indexeddb is being used just ignore localbrowser storage
    // if block_explorer_count.0 > 0 {
    //     loading_init_block_text.set(InitLoadingBlocksState::IndexedDB);
    // } else {
    //     loading_init_block_text.set(InitLoadingBlocksState::LocalBrowserStorage);
    // }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn edge_system(
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut Location), With<Land>>,
    mut edge_event: EventReader<EdgeEvent>,
    mut chunk_set: ResMut<ChunkManager>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    mut update_ui_amount_event: EventWriter<UpdateUiAmount>,
) {
    for edge_e in edge_event.read() {
        // fuck despawn
        for (block_entity, mut block_location) in blocks.iter_mut() {
            if ((block_location.y - edge_e.y).abs() > DESPAWN_TILE_THRESHOLD
                || (block_location.x - edge_e.x).abs() > DESPAWN_TILE_THRESHOLD)
                && !block_location.despawn_status
            {
                block_location.despawn_status = true;
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
                        despawn_status: false,
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

                    let transform = Transform {
                        translation: Vec3::new(
                            TOTAL_TILE_SCALE_SIZE * x as f32,
                            TOTAL_TILE_SCALE_SIZE * y as f32,
                            0.,
                        ),
                        scale: Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.0),
                        ..Default::default()
                    };
                    let mut cmd = commands.spawn((
                        Sprite {
                            color: color_for_tile,
                            texture_atlas: Some(TextureAtlas {
                                layout: texture_atlas_handle_land.layout.clone(),
                                index,
                            }),
                            image: texture_atlas_handle_land.texture.clone(),
                            ..Default::default()
                        },
                        transform,
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

                    // spawn text
                    if zoom_level < TEXT_ZOOM_OUT_MAX {
                        cmd.with_children(|builder| {
                            let slightly_smaller_text_style = TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: (SCALE_FACTOR / 3.0) * 24.0,
                                font_smoothing: FontSmoothing::AntiAliased,
                            };
                            //color: get_text_color(&color_for_tile),

                            let mut text_ent_cmd = builder.spawn((
                                Text2d::new(tile_text),
                                slightly_smaller_text_style,
                                TextLayout::new(JustifyText::Left, LineBreak::WordBoundary),
                                Transform {
                                    translation: Vec3::new(0., 0., 5.),
                                    scale: Vec3::new(1.0 / SCALE_FACTOR, 1.0 / SCALE_FACTOR, 1.0),
                                    ..Default::default()
                                },
                                text_visibility,
                                locationcoord,
                                TileText,
                            ));

                            text_ent_cmd.insert(Aabb {
                                center: Vec3A::ZERO,
                                half_extents: Vec3A::ZERO,
                            });
                        });
                    }

                    let building_color = sanitize_building_color(color_for_sprites.into());

                    // spawn buildings
                    if zoom_level < BUILDING_ZOOM_OUT_MAX {
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
        (&mut Sprite, &Location, Entity, &Children),
        (With<Land>, Without<BuildingStructure>),
    >,
    buildings: Query<(&Location, Entity, &BuildingStructure), Without<Land>>,
    mut event: EventReader<UpdateTileTextureEvent>,
    mut tile_map: ResMut<WorldOwnedTileMap>,
    texture_map: Res<SpriteIndexBuilding>,
    texture_atlas_handle_building: Res<SpriteSheetBuilding>,
    toggle_map: Res<ToggleMap>,
    mut text_q: Query<(&mut Text, &mut TextColor, &Location), With<TileText>>,
    map_mode: Res<MapTileMode>,
    cam_query: Query<&OrthographicProjection, With<Camera>>,
) {
    for tile_vec in event.read() {
        info!("receving update texture event");
        let zoom_level = cam_query.get_single().unwrap().scale;

        let tiles = tile_vec.0.clone();
        let tile_map_from_e: HashMap<u32, TileData> =
            tiles.into_iter().map(|tile| (tile.height, tile)).collect();
        let mut tile_map_from_delta = tile_map_from_e.clone();

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

        for (mut sprite, location, parent_entity, children) in lands.iter_mut() {
            let map_tile_check = tile_map.map.contains_key(&location.ulam);
            let event_tile_check = tile_map_from_e.contains_key(&location.ulam);

            match (map_tile_check, event_tile_check) {
                // both map and event tiles are found
                (true, true) => {
                    // making it where the event is driving not the tile resource
                    let map_tile = tile_map.map.get(&location.ulam).unwrap();
                    let event_tile = tile_map_from_e.get(&location.ulam).unwrap();

                    // see if we should update the world map
                    if map_tile != event_tile {
                        info!("are we getting here 1?");
                        // see if we should update visually
                        if !map_tile.compare_update_fields(event_tile) {
                            info!(
                                "are we getting here 2? map_tile: {:#?}\n\nevent_tile: {:#?}",
                                map_tile, event_tile
                            );
                            let building_sprite_index =
                                *texture_map.0.get(&event_tile.value).unwrap() as usize;

                            let c = ulam::calc_coord::calc_coord(event_tile.height);
                            let mut locationcoord = Location {
                                x: c.x,
                                y: c.y,
                                ulam: event_tile.height,
                                quad: ulam::quad_of_xy(c.x, c.y),
                                selected: false,
                                despawn_status: false,
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

                            info!("is this being triggered? height: {}", event_tile.height);

                            if let Some(atlas) = &mut sprite.texture_atlas {
                                (atlas.index, sprite.color) =
                                    get_index_color(&map_mode, &tile_map, &locationcoord.ulam);
                            }

                            // Not really a big deal to do this everytime because likely after each purchase we will need to configure the buildings differently.
                            /////////////////////

                            let prev_building_type_r = buildings.get(children[0]);
                            let prev_building_type: &BuildingStructure = match prev_building_type_r
                            {
                                Ok(o) => {
                                    info!("is this happening?");
                                    o.2
                                }
                                Err(_) => &BuildingStructure::None,
                            };
                            if zoom_level < BUILDING_ZOOM_OUT_MAX {
                                for (building_location, building_entity, building_type) in
                                    buildings.iter()
                                {
                                    if building_location.ulam == location.ulam
                                        && building_type != prev_building_type
                                    {
                                        //info!("despawning old building stuff");
                                        commands.entity(building_entity).despawn_recursive();
                                    }
                                }

                                // building
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
                                                event_tile.color.into(),
                                            )),
                                            locationcoord,
                                            visibility_building_toggle,
                                        );
                                    });
                            }
                        }
                        //info!("tilemap insert for height: {}", event_tile.height);
                        tile_map.map.insert(event_tile.height, event_tile.clone());
                        tile_map_from_delta.remove(&event_tile.height);
                    }

                    // text
                    let (mut text, mut text_color, loc) = text_q.get_mut(children[0]).unwrap();

                    //for (mut text, loc) in text_q.iter_mut() {
                    if zoom_level < TEXT_ZOOM_OUT_MAX {
                        if let Some(val) = tile_map.map.get(&loc.ulam) {
                            if !*hiding_text {
                                **text_color = get_text_color(&sprite.color);
                                if *showing_value {
                                    **text = val.cost.to_string();
                                } else {
                                    **text = val.height.to_string();
                                }
                            }
                        }
                    }
                }
                (true, false) => {
                    //info!("ignore no event for this");
                }
                // no entry found in map
                (false, true) => {
                    let event_tile = tile_map_from_e.get(&location.ulam).unwrap();

                    // see if we should update visually
                    let building_sprite_index =
                        *texture_map.0.get(&event_tile.value).unwrap() as usize;

                    let c = ulam::calc_coord::calc_coord(event_tile.height);
                    let mut locationcoord = Location {
                        x: c.x,
                        y: c.y,
                        ulam: event_tile.height,
                        quad: ulam::quad_of_xy(c.x, c.y),
                        selected: false,
                        despawn_status: false,
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

                    // the worldmap isn't going to populated at this point so this will not get a good color on the first go when it's (false,true)
                    // so use the event color instead for the first pass. "_"
                    let (ind, _) = get_index_color(&map_mode, &tile_map, &locationcoord.ulam);

                    if let Some(atlas) = &mut sprite.texture_atlas {
                        atlas.index = ind;
                    }

                    sprite.color = event_tile.color;
                    // Not really a big deal to do this everytime because likely after each purchase we will need to configure the buildings differently.
                    /////////////////////

                    let prev_building_type_r = buildings.get(children[0]);
                    let prev_building_type: &BuildingStructure = match prev_building_type_r {
                        Ok(o) => {
                            info!("is this happening?");
                            o.2
                        }
                        Err(_) => &BuildingStructure::None,
                    };

                    if zoom_level < BUILDING_ZOOM_OUT_MAX {
                        for (building_location, building_entity, building_type) in buildings.iter()
                        {
                            if building_location.ulam == location.ulam
                                && building_type != prev_building_type
                            {
                                //info!("despawning old building stuff");
                                commands.entity(building_entity).despawn_recursive();
                            }
                        }

                        // building
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
                                        event_tile.color.into(),
                                    )),
                                    locationcoord,
                                    visibility_building_toggle,
                                );
                            });
                    }

                    //info!("tilemap insert for height: {}", event_tile.height);
                    tile_map.map.insert(event_tile.height, event_tile.clone());
                    tile_map_from_delta.remove(&event_tile.height);
                    // text
                    let (mut text, mut text_color, loc) = text_q.get_mut(children[0]).unwrap();

                    //for (mut text, loc) in text_q.iter_mut() {
                    if zoom_level < TEXT_ZOOM_OUT_MAX {
                        if let Some(val) = tile_map.map.get(&loc.ulam) {
                            if !*hiding_text {
                                **text_color = get_text_color(&sprite.color);
                                if *showing_value {
                                    **text = val.cost.to_string();
                                } else {
                                    **text = val.height.to_string();
                                }
                            }
                        }
                    }
                }
                (false, false) => {
                    //info!("tile isn't loaded on screen");
                }
            }
        }
        info!(
            "need to insert deltas count from map {}",
            tile_map_from_delta.len()
        );
        for (k, v) in tile_map_from_delta.iter() {
            tile_map.map.insert(*k, v.clone());
        }
    }
}

// #[allow(clippy::too_many_arguments)]
// pub fn spawn_block_text_and_buildings_on_zoom_event(
//     asset_server: Res<AssetServer>,
//     building_texture_mapping: Res<SpriteIndexBuilding>,
//     mut commands: Commands,
//     texture_atlas_handle_building: Res<SpriteSheetBuilding>,
//     texture_atlas_handle_land: Res<SpriteSheetLand>,
//     edge: Res<Edge>,
//     mut chunk_set: ResMut<ChunkManager>,
//     tile_map: Res<WorldOwnedTileMap>,
//     toggle_map: Res<ToggleMap>,
//     mut lands: Query<
//         (&mut TextureAtlas, &mut Sprite, &Location, Entity, &Children),
//         (With<Land>, Without<BuildingStructure>),
//     >,
//     mut zoom_event: EventReader<ZoomThresholdEvent>,
// ) {
//     for event_type in zoom_event.read() {
//         match event_type.0 {
//             ZoomSpawnEvent::YesText => {
//                 //spawn_text_for_tile_edgebox(toggle_map, commands, edge, lands, &tile_map);
//                 let text_visibility = if *toggle_map.0.get("showtext").unwrap() {
//                     Visibility::Hidden
//                 } else {
//                     Visibility::Visible
//                 };
//                 let slightly_smaller_text_style = TextStyle {
//                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                     font_size: 24.0,
//                     color: get_text_color(&color_for_tile),
//                 };
//                 let tile_text = if *toggle_map.0.get("showvalues").unwrap() {
//                     locationcoord.ulam.to_string()
//                 } else if *toggle_map.0.get("showheights").unwrap() {
//                     let a = value_from_tile;
//                     if a == 0 {
//                         "".to_string()
//                     } else {
//                         a.to_string()
//                     }
//                 } else {
//                     "somethingwrongvalue".to_string()
//                 };

//                 for (mut texture, mut sprite, location, parent_entity, children) in lands.iter_mut()
//                 {
//                     let text_ent_cmd = commands.spawn((
//                         Text2dBundle {
//                             text: Text {
//                                 sections: vec![TextSection::new(
//                                     tile_text,
//                                     slightly_smaller_text_style.clone(),
//                                 )],
//                                 justify: JustifyText::Left,
//                                 ..Default::default()
//                             },
//                             text_2d_bounds: Text2dBounds { ..default() },
//                             transform: Transform {
//                                 translation: Vec3::new(0., 0., 5.),
//                                 scale: Vec3::new(1.0 / SCALE_FACTOR, 1.0 / SCALE_FACTOR, 1.0),
//                                 ..Default::default()
//                             },
//                             visibility: text_visibility,
//                             ..default()
//                         },
//                         locationcoord,
//                         TileText,
//                     ));
//                     text_ent_cmd.insert(Aabb {
//                         center: Vec3A::ZERO,
//                         half_extents: Vec3A::ZERO,
//                     });

//                     text_ent_cmd.set_parent(parent_entity);
//                 }
//             }
//             ZoomSpawnEvent::YesBuildings => {
//                 // getting whether or not we should spawn buildings as hidden or visible depending on zoom level
//                 let visibility_setting = if *toggle_map.0.get("showbuildings").unwrap() {
//                     Visibility::Hidden
//                 } else {
//                     Visibility::Visible
//                 };

//                 commands
//                     .entity(parent_entity)
//                     .with_children(|child_builder| {
//                         //info!("spawning??!");
//                         spawn_tile_level(
//                             building_sprite_index,
//                             &texture_atlas_handle_building.layout,
//                             &texture_atlas_handle_building.texture,
//                             child_builder,
//                             bevy::prelude::Color::Srgba(sanitize_building_color(
//                                 event_tile.color.into(),
//                             )),
//                             locationcoord,
//                             visibility_building_toggle,
//                         );
//                     });
//             }
//             _ => {
//                 info!("this logic not reachable yet");
//             }
//         }
//     }
// }
pub fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
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
                **text = "Clear".to_string();
                *color = colors.light_color.into();
                border_color.0 = colors.light_color;
                clear_event.send(ClearSelectionEvent);
            }
            Interaction::Hovered => {
                **text = "Clear".to_string();
                *color = colors.accent_color.into();
                border_color.0 = colors.node_color;
            }
            Interaction::None => {
                **text = "Clear".to_string();
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
