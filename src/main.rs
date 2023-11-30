use bevy::{input::mouse::MouseMotion, prelude::*, text::Text2dBounds, utils::HashMap};
use rand::Rng;
use ulam::Quad;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::comms::{load_server_data::api_get_server_tiles, CommsPlugin};

const CHUNK_PIXEL_SIZE: f32 = 400.0;
const TILE_SCALE: f32 = 3.0;
const TILE_PIXEL_SIZE: f32 = 32.0;
const TILE_PADDING_SIZE: f32 = 0.0;
const TOTAL_TILE_SCALE_SIZE: f32 = TILE_PIXEL_SIZE * TILE_SCALE + 4.0;
const CHUNK_TILE_SPAN_COUNT: i32 = (CHUNK_PIXEL_SIZE / TOTAL_TILE_SCALE_SIZE) as i32;
const DESPAWN_TILE_THRESHOLD: i32 = 51 + CHUNK_TILE_SPAN_COUNT * 2;
const CAMERA_SANITY_FACTOR: f32 = 1.25;
const MOVE_VELOCITY_FACTOR: f32 = 10.0;

mod building_config;
mod comms;

use async_channel::{Receiver, Sender};

#[derive(Resource, Clone)]
pub struct TileDataChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct ServerURL(String);

#[derive(Resource, Deref, DerefMut, Clone)]
struct SpriteSheetBgRes(Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut, Clone)]
struct SpriteSheetBuildingRes(Handle<TextureAtlas>);

#[derive(Event, Debug)]
struct UpdateTileTextureEvent;

#[derive(Debug)]
enum EdgeType {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Event, Debug)]
struct EdgeEvent {
    pub edge_type: EdgeType,
    pub x: i32,
    pub y: i32,
}

#[derive(Event)]
struct SpriteSpawnEvent;

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
}

#[derive(Resource, Clone)]
struct ChunkManager {
    pub map: HashMap<u32, bool>,
}

#[derive(Clone)]
pub enum TileResource {
    Wheat,
    Brick,
    Sheep,
    Wood,
    Stone,
    Desert,
    Water,
    Grass,
    Unknown,
}

#[derive(Resource, Clone)]
pub struct TileData {
    pub ln_address: String,
    pub owner: String,
    pub color: Color,
    pub message: String,
    pub resource: TileResource,
    pub hash: String,
    pub amount: u32,
    pub height: u32,
}

#[derive(Resource, Clone)]
pub struct TileMap {
    pub map: HashMap<u32, TileData>,
}

#[derive(Resource, Clone)]
pub struct TextureMap(HashMap<u32, u32>);

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
    pub quad: ulam::Quad,
}

#[derive(Component, Clone, Copy)]
struct Land;

#[derive(Component, Clone, Copy)]
enum BuildingStructure {
    Empty,
    Hut,
    Road,
    RoadCorner,
    FirePit,
}
#[derive(Component, Clone, Copy)]
struct WithinTilePlacement {
    x: f32,
    y: f32,
    z: f32,
    scale: f32,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum CommsState {
    #[default]
    Off,
    On,
}

pub fn main() {
    //game("localusertesting".to_string(), "localhost:8081".to_string());
}
#[wasm_bindgen]
pub fn game12(username: String, server_url: String, ln_address: String) {
    let mut numbers_map = HashMap::new();

    // numbers_map.insert(0, 1500);
    // numbers_map.insert(128, 844);
    // numbers_map.insert(256, 812);
    // numbers_map.insert(512, 3);
    // numbers_map.insert(1024, 778);
    // numbers_map.insert(2048, 79);
    // numbers_map.insert(4096, 785);
    // numbers_map.insert(8192, 37);
    // numbers_map.insert(16384, 789);
    // numbers_map.insert(32768, 857);
    // numbers_map.insert(65536, 799);
    // numbers_map.insert(131072, 73);
    // numbers_map.insert(262144, 64);
    // numbers_map.insert(524288, 61);
    // numbers_map.insert(1048576, 43);

    numbers_map.insert(0, 0);
    numbers_map.insert(128, 1);
    numbers_map.insert(256, 1);
    numbers_map.insert(512, 1);
    numbers_map.insert(1024, 1);
    numbers_map.insert(2048, 1);
    numbers_map.insert(4096, 1);
    numbers_map.insert(8192, 1);
    numbers_map.insert(16384, 1);
    numbers_map.insert(32768, 1);
    numbers_map.insert(65536, 1);
    numbers_map.insert(131072, 1);
    numbers_map.insert(262144, 1);
    numbers_map.insert(524288, 1);
    numbers_map.insert(1048576, 1);

    info!(
        "user: {}\nserver: {}, lnaddress: {}",
        username, server_url, ln_address
    );
    let start_edge = Edge {
        top: EdgeData {
            pixel: CHUNK_PIXEL_SIZE / 2.0,
            tile: CHUNK_TILE_SPAN_COUNT,
        },
        bottom: EdgeData {
            pixel: -CHUNK_PIXEL_SIZE / 2.0,
            tile: -CHUNK_TILE_SPAN_COUNT,
        },
        left: EdgeData {
            pixel: -CHUNK_PIXEL_SIZE / 2.0,
            tile: -CHUNK_TILE_SPAN_COUNT,
        },
        right: EdgeData {
            pixel: CHUNK_PIXEL_SIZE / 2.0,
            tile: CHUNK_TILE_SPAN_COUNT,
        },
    };

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
        .insert_resource(ServerURL(server_url))
        .insert_resource(TextureMap(numbers_map))
        .add_state::<CommsState>()
        .add_plugins(CommsPlugin)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        //.insert_resource(WinitSettings::desktop_app())
        .add_event::<EdgeEvent>()
        .add_event::<SpriteSpawnEvent>()
        .add_event::<UpdateTileTextureEvent>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, api_get_server_tiles)
        .insert_resource(start_edge)
        .insert_resource(ChunkManager {
            map: HashMap::new(),
        })
        .insert_resource(TileMap {
            map: HashMap::new(),
        })
        .add_systems(
            Update,
            (
                zoom_out_button_system,
                zoom_in_button_system,
                mouse_camera_system,
                touch_event_system,
                edge_system,
                //update_tile_textures,
                spawn_block_sprites,
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
                cam_transform.translation += direction
                    * time.delta_seconds()
                    * TILE_SCALE
                    * cam_ortho.scale
                    * MOVE_VELOCITY_FACTOR;
                set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
            }
        }
    }
}

fn touch_event_system(
    //mut touch_events: EventReader<TouchInput>,
    touches: Res<Touches>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
) {
    for touch in touches.iter() {
        for (mut cam_transform, cam_ortho) in camera.iter_mut() {
            let distx;
            let disty;
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
            let direction = Vec3::new(-distx, disty, 0.0);
            cam_transform.translation +=
                direction * time.delta_seconds() * cam_ortho.scale * MOVE_VELOCITY_FACTOR * 4.0;

            set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
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
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
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
    // edge: Res<Edge>,
    // mut chunk_map: ResMut<ChunkManager>,
    // tile_map: Res<TileMap>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
) {
    // ui camera
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("spritesheet/grassdirtbg.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_PIXEL_SIZE, TILE_PIXEL_SIZE),
        11,
        1,
        Some(Vec2::new(TILE_PADDING_SIZE, TILE_PADDING_SIZE)),
        None,
    );
    let texture_handle_buildings = asset_server.load("spritesheet/buildings.png");
    let texture_atlas_building = TextureAtlas::from_grid(
        texture_handle_buildings,
        Vec2::new(TILE_PIXEL_SIZE, TILE_PIXEL_SIZE),
        5,
        1,
        Some(Vec2::new(0., 0.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas_handle_building = texture_atlases.add(texture_atlas_building);

    // let mut spritesheet_texture_atlas_handle_map = HashMap::new();
    // spritesheet_texture_atlas_handle_map.insert("background".to_string());
    // spritesheet_texture_atlas_handle_map.insert("buildings".to_string());

    commands.insert_resource(SpriteSheetBgRes(texture_atlas_handle.clone()));
    commands.insert_resource(SpriteSheetBuildingRes(
        texture_atlas_handle_building.clone(),
    ));

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

    let (tx_tiledata, rx_tiledata) = async_channel::bounded(1);
    commands.insert_resource(TileDataChannel {
        tx: tx_tiledata,
        rx: rx_tiledata,
    });
    sprite_spawn_event.send(SpriteSpawnEvent);
}

#[allow(clippy::too_many_arguments)]
fn edge_system(
    //edge: ResMut<Edge>,
    mut commands: Commands,
    blocks: Query<(Entity, &Location), With<Land>>,
    mut edge_event: EventReader<EdgeEvent>,
    // asset_server: Res<AssetServer>,
    // texture_atlases: Res<Assets<TextureAtlas>>,
    mut chunk_map: ResMut<ChunkManager>,
    // tile_map: Res<TileMap>,
    mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
) {
    for edge_e in edge_event.read() {
        for (block_entity, block_location) in blocks.iter() {
            if (block_location.y - edge_e.y).abs() > DESPAWN_TILE_THRESHOLD
                || (block_location.x - edge_e.x).abs() > DESPAWN_TILE_THRESHOLD
            {
                info!("despawning");
                let ulam_i = ulam::value_of_xy(block_location.x, block_location.y);
                commands.entity(block_entity).despawn_recursive();
                chunk_map.map.remove(&ulam_i);
            }
        }

        debug!("reached edge: {:?}", edge_e.edge_type);

        // spawn_block_sprites(
        //     &mut commands,
        //     &asset_server,
        //     &texture_atlas_handle,
        //     edge.clone(),
        //     &mut chunk_map,
        //     &tile_map,
        // );
        sprite_spawn_event.send(SpriteSpawnEvent);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_block_sprites(
    asset_server: Res<AssetServer>,
    texture_map: Res<TextureMap>,
    mut sprite_spawn_event: EventReader<SpriteSpawnEvent>,
    mut commands: Commands,
    texture_atlas_handle_bg: Res<SpriteSheetBgRes>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
    //mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    edge: Res<Edge>,
    mut chunk_map: ResMut<ChunkManager>,
    tile_map: Res<TileMap>,
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
        let mut color_for_tile;

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

                    let mut rng = rand::thread_rng();
                    let base_sprite_index: usize = rng.gen_range(0..=10);

                    //let top_sprite_index;
                    let r: f32; //= rng.gen_range(0.0..=1.0);
                    let g: f32; //= rng.gen_range(0.0..=1.0);
                    let b: f32; //= rng.gen_range(0.0..=1.0);
                                //let amount_index_num;
                    if tile_map.map.contains_key(&locationcoord.ulam) {
                        let amount_from_tile =
                            tile_map.map.get(&locationcoord.ulam).unwrap().amount;
                        building_sprite_index =
                            *texture_map.0.get(&amount_from_tile).unwrap() as usize;

                        color_for_tile = tile_map.map.get(&locationcoord.ulam).unwrap().color;
                        r = 1.0;
                        g = 1.0;
                        b = 1.0;

                        //top_sprite_index = texture_map.0.get(&amount_index_num).unwrap();
                    } else {
                        r = 0.5;
                        g = 0.5;
                        b = 0.5;
                        building_sprite_index = 0;
                        color_for_tile = Color::Rgba {
                            red: 0.5,
                            green: 0.5,
                            blue: 0.5,
                            alpha: 1.0,
                        };
                        //top_sprite_index = texture_map.0.get(&0).unwrap();
                    }
                    // let textureatlashandle_background: &Handle<TextureAtlas> =
                    //     texture_atlas_handle_.get("background").unwrap();
                    // let textureatlashandle_buildings: &Handle<TextureAtlas> =
                    //     texture_atlas_handle.get("buildings").unwrap();
                    commands
                        .spawn((
                            SpriteSheetBundle {
                                texture_atlas: texture_atlas_handle_bg.0.clone(), //textureatlashandle.clone(),
                                sprite: TextureAtlasSprite {
                                    color: Color::Rgba {
                                        red: r,
                                        green: g,
                                        blue: b,
                                        alpha: 1.0,
                                    },
                                    index: base_sprite_index,
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
                            building_config::road::spawn(
                                &texture_atlas_handle_building.0.clone(),
                                builder,
                                Color::rgba(1.0, 1.0, 1.0, 2.0),
                                locationcoord,
                            );
                        })
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
                                text_2d_bounds: Text2dBounds { ..default() },
                                transform: Transform {
                                    translation: Vec3::new(0., 0., 3.),
                                    scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                                    ..Default::default()
                                },
                                ..default()
                            });
                        })
                        .with_children(|builder| {
                            match building_sprite_index {
                                1 => {
                                    building_config::level1::spawn(
                                        &texture_atlas_handle_building.0.clone(),
                                        builder,
                                        color_for_tile,
                                        locationcoord,
                                    );
                                }
                                _ => {
                                    // do nothing
                                }
                            }

                            // builder.spawn((
                            //     SpriteSheetBundle {
                            //         texture_atlas: textureatlashandle_buildings.clone(),
                            //         sprite: TextureAtlasSprite {
                            //             color: color_for_tile,
                            //             index: building_sprite_index,
                            //             ..Default::default()
                            //         },
                            //         transform: Transform {
                            //             translation: Vec3::new(5., 0., 3.),
                            //             scale: Vec3::new(1.0 / TILE_SCALE, 1.0 / TILE_SCALE, 1.0),
                            //             ..Default::default()
                            //         },
                            //         ..Default::default()
                            //     },
                            //     BuildingStructure::Hut,
                            //     locationcoord,
                            // ));
                        });
                }
            }
        }
    }
}

fn set_camera_tile_bounds(
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
        info!("new left {}", edge.left.pixel);

        if camera_vec3.x < edge.left.pixel * CAMERA_SANITY_FACTOR {
            info!("adjust left?");
            camera_vec3.x = edge.left.pixel;
        }
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
        info!("new right {}", edge.right.pixel);

        if camera_vec3.x > edge.right.pixel * CAMERA_SANITY_FACTOR {
            info!("adjust right?");
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

        info!("new top {}", edge.top.pixel);
        if camera_vec3.y > edge.top.pixel * CAMERA_SANITY_FACTOR {
            info!("adjust top");
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
        info!("new bottom {}", edge.bottom.pixel);
        if camera_vec3.y < edge.bottom.pixel * CAMERA_SANITY_FACTOR {
            info!("adjust bottom");
            camera_vec3.y = edge.bottom.pixel;
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn update_tile_textures(
    //mut commands: Commands,
    //mut blocks: Query<&mut Location>,
    mut block_buildings: Query<
        (&mut TextureAtlasSprite, &Location),
        (With<BuildingStructure>, Without<Land>),
    >,
    mut lands: Query<
        (&mut TextureAtlasSprite, &Location),
        (With<Land>, Without<BuildingStructure>),
    >,
    mut event: EventReader<UpdateTileTextureEvent>,
    tile_map: Res<TileMap>,
    //chunk_map: Res<ChunkManager>,
    texture_map: Res<TextureMap>,
) {
    for _e in event.read() {
        for (mut texture, location) in block_buildings.iter_mut() {
            if tile_map.map.contains_key(&location.ulam) {
                let a = tile_map.map.get(&location.ulam).unwrap();
                texture.color = a.color;
                let b = a.amount;
                texture.index = *texture_map.0.get(&b).unwrap() as usize;
            }
        }

        for (mut texture, location) in lands.iter_mut() {
            if tile_map.map.contains_key(&location.ulam) {
                texture.color = Color::Rgba {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                    alpha: 1.,
                };
            }
        }
        info!("updated textures");
    }
}
