use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        touch::Touch,
        touchpad::{TouchpadMagnify, TouchpadRotate},
    },
    prelude::*,
    text::{scale_value, Text2dBounds},
    ui::RelativeCursorPosition,
    utils::HashMap,
    winit::WinitSettings,
};
use rand::Rng;
// multiple of 100's is best for the tilemap
const CHUNK_PIXEL_SIZE: f32 = 400.0;
const TILE_SCALE: f32 = 3.0;
const TILE_PIXEL_SIZE: f32 = 32.0;
const TILE_PADDING_SIZE: f32 = 2.0;
//const TOTAL_TILE_SIZE: f32 = TILE_PIXEL_SIZE + TILE_PADDING_SIZE;
const TOTAL_TILE_SCALE_SIZE: f32 = TILE_PIXEL_SIZE * TILE_SCALE + 4.0;
const CHUNK_TILE_SPAN_COUNT: i32 = (CHUNK_PIXEL_SIZE / TOTAL_TILE_SCALE_SIZE) as i32;
//const GRID_SIZE_WIDTH: i32 = 50;
//const GRID_SIZE_HEIGHT: i32 = 50;
const BORDER_PIXEL: f32 = CHUNK_TILE_SPAN_COUNT as f32 * TOTAL_TILE_SCALE_SIZE;
const DESPAWN_TILE_THRESHOLD: i32 = CHUNK_TILE_SPAN_COUNT * 10;
// first_pressed_position
// get_pressed
// just_pressed
// just_released

#[derive(Resource, Deref, DerefMut, Clone)]
struct SpriteSheetRes(Handle<TextureAtlas>);

enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
    Middle,
}

#[derive(Event)]
struct EdgeEvent {
    pub edge_type: EdgeType,
    pub x: i32,
    pub y: i32,
}

#[derive(Clone)]
struct EdgeData {
    pub tile: i32,
    pub pixel: f32,
}

#[derive(Resource, Clone)]
struct Edge {
    pub top: EdgeData,
    pub bottom: EdgeData,
    pub left: EdgeData,
    pub right: EdgeData,
    pub middle: EdgeData,
}

#[derive(Resource, Clone)]
struct ChunkManager {
    pub map: HashMap<u32, bool>,
}

#[derive(Debug)]
struct SpawnDiffData {
    pub xstart: i32,
    pub xend: i32,
    pub ystart: i32,
    pub yend: i32,
}

#[derive(Component)]
struct ZoomOut;

#[derive(Component)]
struct ZoomIn;

#[derive(Component, Clone, Copy)]
struct Location {
    pub x: i32,
    pub y: i32,
    pub ulam: u32,
}

fn main() {
    App::new()
        //.add_plugins(DefaultPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                //resolution: [width as f32, height as f32].into(),
                fit_canvas_to_parent: true,
                title: "SatoshiSettlers".to_string(),
                ..default()
            }),
            ..default()
        }))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        //.insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_event::<EdgeEvent>()
        .insert_resource(Edge {
            top: EdgeData {
                pixel: BORDER_PIXEL,
                tile: CHUNK_TILE_SPAN_COUNT,
            },
            bottom: EdgeData {
                pixel: -BORDER_PIXEL,
                tile: -CHUNK_TILE_SPAN_COUNT,
            },
            left: EdgeData {
                pixel: -BORDER_PIXEL,
                tile: -CHUNK_TILE_SPAN_COUNT,
            },
            right: EdgeData {
                pixel: BORDER_PIXEL,
                tile: CHUNK_TILE_SPAN_COUNT,
            },
            middle: EdgeData {
                pixel: 0.0,
                tile: 0,
            },
        })
        .insert_resource(ChunkManager {
            map: HashMap::new(),
        })
        .add_systems(
            Update,
            (
                zoom_out_button_system,
                zoom_in_button_system,
                mouse_camera_system,
                //touch_event_system,
                edge_system,
            ), //, print_mouse_events_system, touch_event_system
        )
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[allow(clippy::too_many_arguments)]
fn mouse_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
) {
    if mouse.pressed(MouseButton::Middle) || mouse.pressed(MouseButton::Left) {
        for event in mouse_motion_events.read() {
            for (mut cam_transform, cam_ortho) in camera.iter_mut() {
                let direction = Vec3::new(-event.delta.x, event.delta.y, 0.0);
                cam_transform.translation +=
                    direction * time.delta_seconds() * TILE_SCALE * cam_ortho.scale * 20.0;
                if cam_transform.translation.x < edge.left.pixel {
                    cam_transform.translation.x = edge.left.pixel;

                    edge.left.pixel -= CHUNK_PIXEL_SIZE;
                    edge.left.tile -= CHUNK_TILE_SPAN_COUNT;
                    edge.right.pixel -= CHUNK_PIXEL_SIZE;
                    edge.right.tile -= CHUNK_TILE_SPAN_COUNT;

                    info!("sending left?");
                    edge_event.send(EdgeEvent {
                        edge_type: EdgeType::Left,
                        x: edge.left.tile,
                        y: (edge.top.tile + edge.bottom.tile) / 2,
                    });
                    info!("new left {}", edge.left.pixel);
                }
                if cam_transform.translation.x > edge.right.pixel {
                    cam_transform.translation.x = edge.right.pixel;
                    edge.right.pixel += CHUNK_PIXEL_SIZE;
                    edge.right.tile += CHUNK_TILE_SPAN_COUNT;
                    edge.left.pixel += CHUNK_PIXEL_SIZE;
                    edge.left.tile += CHUNK_TILE_SPAN_COUNT;
                    info!("sending right?");
                    edge_event.send(EdgeEvent {
                        edge_type: EdgeType::Right,
                        x: edge.right.tile,
                        y: (edge.top.tile + edge.bottom.tile) / 2,
                    });
                    info!("new right {}", edge.right.pixel);
                }
                if cam_transform.translation.y > edge.top.pixel {
                    cam_transform.translation.y = edge.top.pixel;
                    edge.top.pixel += CHUNK_PIXEL_SIZE;
                    edge.top.tile += CHUNK_TILE_SPAN_COUNT;
                    edge.bottom.pixel += CHUNK_PIXEL_SIZE;
                    edge.bottom.tile += CHUNK_TILE_SPAN_COUNT;

                    info!("before local");
                    info!("sending top?");
                    edge_event.send(EdgeEvent {
                        edge_type: EdgeType::Top,
                        x: (edge.left.tile + edge.right.tile) / 2,
                        y: edge.top.tile,
                    });

                    info!("new top {}", edge.top.pixel);
                }
                if cam_transform.translation.y < edge.bottom.pixel {
                    cam_transform.translation.y = edge.bottom.pixel;
                    edge.bottom.pixel -= CHUNK_PIXEL_SIZE;
                    edge.bottom.tile -= CHUNK_TILE_SPAN_COUNT;
                    edge.top.pixel -= CHUNK_PIXEL_SIZE;
                    edge.top.tile -= CHUNK_TILE_SPAN_COUNT;
                    info!("sending bottom?");
                    edge_event.send(EdgeEvent {
                        edge_type: EdgeType::Bottom,
                        x: (edge.left.tile + edge.right.tile) / 2,
                        y: edge.bottom.tile,
                    });
                    info!("new bottom {}", edge.bottom.pixel);
                }
            }
        }
    }
}

fn touch_event_system(
    mut touch_events: EventReader<TouchInput>,
    touches: Res<Touches>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
) {
    for touch in touches.iter() {
        for (mut cam_transform, cam_ortho) in camera.iter_mut() {
            // let direction = Vec3::new(-touch.delta().x, touch.delta().y, 0.0);
            // transform.translation += direction * time.delta_seconds() * 16.0;
            let mut distx = 0.0;
            let mut disty = 0.0;
            //info!("distance {}", touch.distance());
            //info!("delta {}", );
            if touch.delta().x > 0.5 && touch.delta().x < 2.0 {
                distx = 2.0;
            } else if touch.delta().x < -0.5 && touch.delta().x > -2.0 {
                distx = -2.0;
            } else if touch.delta().x > 20.0 {
                distx = 20.0;
            } else if touch.delta().x < -20.0 {
                distx = -20.0;
            } else {
                distx = touch.delta().x;
            }

            if touch.delta().y > 0.5 && touch.delta().y < 2.0 {
                disty = 2.0;
            } else if touch.delta().y < -0.5 && touch.delta().y > -2.0 {
                disty = -2.0;
            } else if touch.delta().y > 20.0 {
                disty = 20.0;
            } else if touch.delta().y < -20.0 {
                disty = -20.0;
            } else {
                disty = touch.delta().y;
            }
            // info!(
            //     "celta [{} => {}, {} => {}]",
            //     touch.delta().x,
            //     distx,
            //     touch.delta().y,
            //     disty
            // );
            //let direction = Vec3::new(-touch.delta().x, touch.delta().y, 0.0);
            let direction = Vec3::new(-distx, disty, 0.0);
            cam_transform.translation += direction * time.delta_seconds() * 21.0 * cam_ortho.scale;
            if cam_transform.translation.x < -BORDER_PIXEL {
                cam_transform.translation.x = -BORDER_PIXEL
            }
            if cam_transform.translation.x > BORDER_PIXEL {
                cam_transform.translation.x = BORDER_PIXEL
            }
            if cam_transform.translation.y > BORDER_PIXEL {
                cam_transform.translation.y = BORDER_PIXEL
            }
            if cam_transform.translation.y < -BORDER_PIXEL {
                cam_transform.translation.y = -BORDER_PIXEL
            }
        }
    }
    // for event in touch_events.read() {
    //     info!("{:?}", event);
    //     info!("{:?}", touches.);
    // }
}

#[allow(clippy::type_complexity)]
fn zoom_out_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<ZoomOut>),
    >,
    mut text_query: Query<&mut Text>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "-".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                for mut ortho in cam_query.iter_mut() {
                    ortho.scale += 0.25;
                    info!("{}", ortho.scale);
                    if ortho.scale > 20.0 {
                        ortho.scale = 10.0;
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
fn zoom_in_button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<ZoomIn>),
    >,
    mut text_query: Query<&mut Text>,
    mut cam_query: Query<(&mut OrthographicProjection), With<Camera>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                text.sections[0].value = "+".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                for mut ortho in cam_query.iter_mut() {
                    ortho.scale -= 0.25;
                    if ortho.scale < 0.5 {
                        ortho.scale = 0.5;
                    }
                    info!("{}", ortho.scale);
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    edge: Res<Edge>,
    mut chunk_map: ResMut<ChunkManager>,
) {
    // ui camera
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("spritesheet/background-extruded.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_PIXEL_SIZE, TILE_PIXEL_SIZE),
        32,
        47,
        Some(Vec2::new(TILE_PADDING_SIZE, TILE_PADDING_SIZE)),
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheetRes(texture_atlas_handle.clone()));

    spawn_block_sprites(
        &mut commands,
        &asset_server,
        &texture_atlas_handle,
        //EdgeType::Middle,
        edge.clone(),
        &mut chunk_map,
    );

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
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
                        ..default()
                    },
                    ZoomOut,
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
                    ZoomIn,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "+",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

#[allow(clippy::too_many_arguments)]
fn edge_system(
    edge: ResMut<Edge>,
    mut commands: Commands,
    blocks: Query<(Entity, &Location)>,
    mut edge_event: EventReader<EdgeEvent>,
    asset_server: Res<AssetServer>,
    texture_atlas_handle: Res<SpriteSheetRes>,
    mut chunk_map: ResMut<ChunkManager>,
) {
    for edge_e in edge_event.read() {
        for (block_entity, block_location) in blocks.iter() {
            // info!(
            //     "blocklocation_y: {}, edge_y: {}, locklocation_x: {}, edge_x: {}, threshold: {}",
            //     block_location.y, edge_e.y, block_location.x, edge_e.x, DESPAWN_TILE_THRESHOLD
            // );
            if (block_location.y - edge_e.y).abs() > DESPAWN_TILE_THRESHOLD
                || (block_location.x - edge_e.x).abs() > DESPAWN_TILE_THRESHOLD
            {
                let ulam_i = ulam::value_of_xy(block_location.x, block_location.y);
                commands.entity(block_entity).despawn_recursive();
                let r = &chunk_map.map.remove(&ulam_i);
                info!("despawning old tiles");
            }
        }
        // info!(
        //     "tile top: {}, tile pixel: {}",
        //     edge.top.tile, edge.top.pixel
        // );
        spawn_block_sprites(
            &mut commands,
            &asset_server,
            &texture_atlas_handle,
            edge.clone(),
            &mut chunk_map,
        );

        // match &edge_e.edge_type {
        //     EdgeType::Top => {
        //         info!("despawning bottom.. reached top");
        //         for (block_entity, block_location) in blocks.iter() {
        //             if (block_location.y - edge_e.y) > DESPAWN_TILE_THRESHOLD
        //                 || (block_location.x - edge_e.x) > DESPAWN_TILE_THRESHOLD
        //             //block_location.y * full_tile_pixel < (edge.bottom.pixel / TILE_SCALE) as i32
        //             {
        //                 commands.entity(block_entity).despawn_recursive();
        //             }
        //         }
        //         info!(
        //             "tile top: {}, tile pixel: {}",
        //             edge.top.tile, edge.top.pixel
        //         );
        //         spawn_block_sprites(
        //             &mut commands,
        //             &asset_server,
        //             &texture_atlas_handle,
        //             EdgeType::Top,
        //             edge.clone(),
        //         );
        //     }
        //     EdgeType::Bottom => {
        //         info!("despawning top.. reached bottom");
        //         for (block_entity, block_location) in blocks.iter() {
        //             if block_location.y * full_tile_pixel > (edge.top.pixel / TILE_SCALE) as i32 {
        //                 commands.entity(block_entity).despawn_recursive();
        //             }
        //         }
        //         info!(
        //             "tile bottom: {}, tile pixel: {}",
        //             edge.bottom.tile, edge.bottom.pixel
        //         );
        //         spawn_block_sprites(
        //             &mut commands,
        //             &asset_server,
        //             &texture_atlas_handle,
        //             EdgeType::Bottom,
        //             edge.clone(),
        //         );
        //     }
        //     EdgeType::Left => {
        //         info!("despawning right.. reached left");
        //         for (block_entity, block_location) in blocks.iter() {
        //             if block_location.x * full_tile_pixel > (edge.right.pixel / TILE_SCALE) as i32 {
        //                 commands.entity(block_entity).despawn_recursive();
        //             }
        //         }
        //         info!(
        //             "tile left: {}, tile pixel: {}",
        //             edge.left.tile, edge.left.pixel
        //         );
        //         spawn_block_sprites(
        //             &mut commands,
        //             &asset_server,
        //             &texture_atlas_handle,
        //             EdgeType::Left,
        //             edge.clone(),
        //         );
        //     }
        //     EdgeType::Right => {
        //         info!("despawning left.. reached right");
        //         for (block_entity, block_location) in blocks.iter() {
        //             if block_location.x * full_tile_pixel < (edge.left.pixel / TILE_SCALE) as i32 {
        //                 commands.entity(block_entity).despawn_recursive();
        //             }
        //         }
        //         info!(
        //             "tile right: {}, tile pixel: {}",
        //             edge.right.tile, edge.right.pixel
        //         );
        //         spawn_block_sprites(
        //             &mut commands,
        //             &asset_server,
        //             &texture_atlas_handle,
        //             EdgeType::Right,
        //             edge.clone(),
        //         );
        //     }
        //     EdgeType::Middle => {
        //         info!("this shouldnt be reached wtf middle?");
        //         // for (block_entity, block_location) in blocks.iter() {
        //         //     if block_location.x * full_tile_pixel < (edge.left.pixel / TILE_SCALE) as i32 {
        //         //         commands.entity(block_entity).despawn_recursive();
        //         //         spawn_block_sprites(
        //         //             &mut commands,
        //         //             &asset_server,
        //         //             &texture_atlas_handle,
        //         //             EdgeType::Right,
        //         //             edge.clone(),
        //         //         );
        //         //     }
        //         // }
        //     }
        // }
    }
}

fn spawn_block_sprites(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_handle: &Handle<TextureAtlas>,
    //edge_type: EdgeType,
    edge_limit: Edge,
    chunk_map: &mut ResMut<ChunkManager>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 24.0,
        color: Color::WHITE,
    };
    // info!(
    //     "before middles top: {}, bottom: {}, left: {}, right: {}, chunk_tile_span_count: {}",
    //     edge_limit.top.tile,
    //     edge_limit.bottom.tile,
    //     edge_limit.left.tile,
    //     edge_limit.right.tile,
    //     CHUNK_TILE_SPAN_COUNT
    // );

    let middle_y = (edge_limit.top.tile + edge_limit.bottom.tile) / 2;
    let middle_x = (edge_limit.left.tile + edge_limit.right.tile) / 2;

    //info!("middle_y: {}, middle_x: {}", middle_y, middle_x);
    let spawn_diff = SpawnDiffData {
        xstart: middle_x - CHUNK_TILE_SPAN_COUNT,
        xend: middle_x + CHUNK_TILE_SPAN_COUNT,
        ystart: middle_y - CHUNK_TILE_SPAN_COUNT,
        yend: middle_y + CHUNK_TILE_SPAN_COUNT,
    };

    // for loop for spawning
    // let spawn_diff = match edge_type {
    //     EdgeType::Top => SpawnDiffData {
    //         xstart: edge_limit.left.tile,
    //         xend: edge_limit.right.tile,
    //         ystart: edge_limit.top.tile - CHUNK_TILE_SPAN_COUNT,
    //         yend: edge_limit.top.tile - 1,
    //     },
    //     EdgeType::Bottom => SpawnDiffData {
    //         xstart: edge_limit.left.tile + 1,
    //         xend: edge_limit.right.tile,
    //         ystart: edge_limit.bottom.tile,
    //         yend: edge_limit.bottom.tile + CHUNK_TILE_SPAN_COUNT,
    //     },
    //     EdgeType::Left => SpawnDiffData {
    //         xstart: edge_limit.left.tile,
    //         xend: edge_limit.left.tile + CHUNK_TILE_SPAN_COUNT - 1,
    //         ystart: edge_limit.bottom.tile,
    //         yend: edge_limit.top.tile,
    //     },
    //     EdgeType::Right => SpawnDiffData {
    //         xstart: edge_limit.right.tile - CHUNK_TILE_SPAN_COUNT + 1,
    //         xend: edge_limit.right.tile,
    //         ystart: edge_limit.bottom.tile,
    //         yend: edge_limit.top.tile,
    //     },
    //     EdgeType::Middle => SpawnDiffData {
    //         xstart: edge_limit.left.tile,
    //         xend: edge_limit.right.tile,
    //         ystart: edge_limit.bottom.tile,
    //         yend: edge_limit.top.tile,
    //     },
    // };

    info!("spawning {:#?}", spawn_diff);
    for x in spawn_diff.xstart..=spawn_diff.xend {
        for y in spawn_diff.ystart..=spawn_diff.yend {
            let ulam_i = ulam::value_of_xy(x, y);
            if !chunk_map.map.contains_key(&ulam_i) {
                chunk_map.map.insert(ulam_i, true);

                //info!("spawning: x: {}, y: {}", x, y);
                let locationcoord = Location {
                    x,
                    y,
                    ulam: ulam::value_of_xy(x, y),
                };

                let mut rng = rand::thread_rng();
                let r: f32 = rng.gen_range(0.0..=1.0);
                let g: f32 = rng.gen_range(0.0..=1.0);
                let b: f32 = rng.gen_range(0.0..=1.0);

                let ranindex: usize = rng.gen_range(194..=222);

                commands
                    .spawn((
                        SpriteSheetBundle {
                            texture_atlas: texture_atlas_handle.clone(),
                            sprite: TextureAtlasSprite {
                                color: Color::Rgba {
                                    red: 1.0,
                                    green: 1.0,
                                    blue: 1.0,
                                    alpha: 1.0,
                                },
                                index: ranindex,
                                ..Default::default()
                            },
                            // Sprite {
                            //     color: Color::rgb(r, g, b),
                            //     custom_size: Some(Vec2::new(100.0, 100.0)),
                            //     ..default()
                            // },
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
                    ))
                    .with_children(|builder| {
                        builder.spawn(Text2dBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    format!("{}", locationcoord.ulam),
                                    slightly_smaller_text_style.clone(),
                                )],
                                alignment: TextAlignment::Left,
                                ..Default::default()
                            },
                            text_2d_bounds: Text2dBounds {
                                // Wrap text in the rectangle
                                // size: box_size,
                                ..default()
                            },
                            // ensure the text is drawn on top of the box
                            transform: Transform {
                                translation: Vec3::Z,
                                scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                                ..Default::default()
                            },
                            ..default()
                        });
                    });
            }
        }
    }
}
