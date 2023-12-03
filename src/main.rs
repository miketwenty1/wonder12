//use bevy::asset::AssetMetaCheck;
use bevy::window::PrimaryWindow;
use bevy::{input::mouse::MouseMotion, prelude::*, text::Text2dBounds, utils::HashMap};
use building_config::spawn_tile_level;
use rand::Rng;
use ulam::Quad;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::comms::{load_server_data::api_get_server_tiles, CommsPlugin};

const CHUNK_PIXEL_SIZE: f32 = 400.0;
const TILE_SCALE: f32 = 3.0;
const TILE_PIXEL_SIZE: f32 = 32.0;
//const TILE_PADDING_SIZE: f32 = 0.0;
const TOTAL_TILE_SCALE_SIZE: f32 = TILE_PIXEL_SIZE * TILE_SCALE + 4.0;
const CHUNK_TILE_SPAN_COUNT: i32 = (CHUNK_PIXEL_SIZE / TOTAL_TILE_SCALE_SIZE) as i32;
const DESPAWN_TILE_THRESHOLD: i32 = 51 + CHUNK_TILE_SPAN_COUNT * 2;
const CAMERA_SANITY_FACTOR: f32 = 1.25;
const MOVE_VELOCITY_FACTOR: f32 = 10.0;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

mod building_config;
mod comms;

use async_channel::{Receiver, Sender};

#[derive(Component)]
struct UiNode;

#[derive(Component)]
struct Selected;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource, Clone)]
pub struct TileDataChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

#[derive(Resource, Clone)]
pub struct ServerURL(String);

#[derive(Resource, Clone, Copy)]
pub struct LastSelectedTile(i32, i32);

#[derive(Resource, Deref, DerefMut, Clone)]
struct SpriteSheetBgRes(Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut, Clone)]
struct SpriteSheetBuildingRes(Handle<TextureAtlas>);

#[derive(Event, Debug)]
struct UpdateTileTextureEvent;

#[derive(Event, Debug)]
struct SelectTileEvent(i32, i32);

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
pub struct SpriteIndexBuilding(HashMap<u32, u32>);

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

#[derive(Component, Clone, Copy, Debug)]
struct Location {
    pub x: i32,
    pub y: i32,
    pub ulam: u32,
    pub quad: ulam::Quad,
    pub selected: bool,
}

#[derive(Component, Clone, Copy)]
struct Land;

#[derive(Component, Clone, Copy)]
enum BuildingStructure {
    //Empty,
    Hut,
    DirtRoad,
    //DirtRoadCorner,
    //DirtRoad2,
    //DirtRoadCorner2,
    FirePit,
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

    numbers_map.insert(0, 0);
    numbers_map.insert(128, 1);
    numbers_map.insert(256, 2);
    numbers_map.insert(512, 3);
    numbers_map.insert(1024, 3);
    numbers_map.insert(2048, 3);
    numbers_map.insert(4096, 3);
    numbers_map.insert(8192, 3);
    numbers_map.insert(16384, 3);
    numbers_map.insert(32768, 3);
    numbers_map.insert(65536, 3);
    numbers_map.insert(131072, 3);
    numbers_map.insert(262144, 3);
    numbers_map.insert(524288, 3);
    numbers_map.insert(1048576, 3);

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
        .insert_resource(SpriteIndexBuilding(numbers_map))
        //.insert_resource(AssetMetaCheck::Never)
        .add_state::<CommsState>()
        .add_plugins(CommsPlugin)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        //.insert_resource(WinitSettings::desktop_app())
        .add_event::<EdgeEvent>()
        .add_event::<SpriteSpawnEvent>()
        .add_event::<UpdateTileTextureEvent>()
        .add_event::<SelectTileEvent>()
        .add_systems(Startup, setup) //setupcoords
        .add_systems(PostStartup, api_get_server_tiles)
        .insert_resource(start_edge)
        .insert_resource(ChunkManager {
            map: HashMap::new(),
        })
        .insert_resource(TileMap {
            map: HashMap::new(),
        })
        .insert_resource(LastSelectedTile(1_000_000, 1_000_000))
        .add_systems(
            Update,
            (
                zoom_out_button_system,
                zoom_in_button_system,
                mouse_camera_system,
                //touch_event_system,
                edge_system,
                //update_tile_textures,
                spawn_block_sprites,
                //animate_sprites,
                //select_tile,
                //my_cursor_system,
            ), //, print_mouse_events_system,
        )
        .run();
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
    let texture_atlas_bg = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_PIXEL_SIZE, TILE_PIXEL_SIZE),
        11,
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
fn mouse_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut q_camera: Query<
        (
            &mut Transform,
            &OrthographicProjection,
            &Camera,
            &GlobalTransform,
        ),
        With<Camera>,
    >,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<SelectTileEvent>,
    //mut last_selected_tile: ResMut<LastSelectedTile>,
    //location_query: Query<&Location>,
) {
    if mouse.pressed(MouseButton::Middle) || mouse.pressed(MouseButton::Left) {
        for (mut cam_transform, cam_ortho, camera, camera_transform) in q_camera.iter_mut() {
            let window = q_window.single();
            // let height = window.resolution.height();
            // let width = window.resolution.width();
            // check if the cursor is inside the window and get its position
            // then, ask bevy to convert into world coordinates, and truncate to discard Z
            if let Some(world_position) = window
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                //mycoords.0 = world_position;
                let x: i32 = if world_position.x >= 0.0 {
                    ((world_position.x + TOTAL_TILE_SCALE_SIZE / 2. - 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                } else {
                    ((world_position.x - TOTAL_TILE_SCALE_SIZE / 2. + 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                };

                let y: i32 = if world_position.y >= 0.0 {
                    ((world_position.y + TOTAL_TILE_SCALE_SIZE / 2. - 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                } else {
                    ((world_position.y - TOTAL_TILE_SCALE_SIZE / 2. + 1.) / TOTAL_TILE_SCALE_SIZE)
                        as i32
                };

                //info!("mouse World coords: {}/{}", x, y);

                if mouse.just_pressed(MouseButton::Left) {
                    // for location in location_query.iter() {
                    //     if location.x == x && location.y == y {
                    //info!("send mouse select");
                    select_tile_event.send(SelectTileEvent(x, y));
                    //   }
                    //}
                    //*last_selected_tile = LastSelectedTile(x, y);
                }
            }

            for event in mouse_motion_events.read() {
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

#[allow(clippy::too_many_arguments)]
fn touch_event_system(
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

            info!(
                "direction: {}, timedelta: {}, camscale: {}",
                direction,
                time.delta_seconds(),
                cam_ortho.scale
            );
            set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);

            if touches.just_pressed(touch.id()) {
                info!("send touch select");
                select_tile_event.send(SelectTileEvent(x, y));
                //*last_selected_tile = LastSelectedTile(x, y);
            }

            info!("touch World coords: {}/{}", x, y);
        }
    }
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

        sprite_spawn_event.send(SpriteSpawnEvent);
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_block_sprites(
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
        let mut color_for_sprites;
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

                    let mut rng = rand::thread_rng();
                    let base_sprite_index: usize = rng.gen_range(0..=10);

                    if tile_map.map.contains_key(&locationcoord.ulam) {
                        let amount_from_tile =
                            tile_map.map.get(&locationcoord.ulam).unwrap().amount;
                        building_sprite_index =
                            *texture_map.0.get(&amount_from_tile).unwrap() as usize;

                        color_for_sprites = tile_map.map.get(&locationcoord.ulam).unwrap().color;
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
                            spawn_tile_level(
                                building_sprite_index,
                                &texture_atlas_handle_building.0.clone(),
                                builder,
                                color_for_sprites,
                                locationcoord,
                            );
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

        // if camera_vec3.x < edge.left.pixel * CAMERA_SANITY_FACTOR {
        //     info!("adjust left?");
        //     camera_vec3.x = edge.left.pixel;
        // }
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
    mut commands: Commands,
    mut lands: Query<
        (&mut TextureAtlasSprite, &Location, Entity),
        (With<Land>, Without<BuildingStructure>),
    >,
    mut event: EventReader<UpdateTileTextureEvent>,
    tile_map: Res<TileMap>,
    texture_map: Res<SpriteIndexBuilding>,
    texture_atlas_handle_building: Res<SpriteSheetBuildingRes>,
) {
    for _e in event.read() {
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
                let mut rng = rand::thread_rng();
                let base_sprite_index: usize = rng.gen_range(0..=10);

                texture.index = base_sprite_index; //*texture_map.0.get(&base_sprite_index).unwrap() as usize;

                commands
                    .entity(parent_entity)
                    .with_children(|child_builder| {
                        spawn_tile_level(
                            building_sprite_index,
                            &texture_atlas_handle_building.0.clone(),
                            child_builder,
                            tile_data.color,
                            locationcoord,
                        );
                    });
            }
        }
        info!("updated textures");
    }
}

fn animate_sprites(
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

#[allow(clippy::type_complexity)]
fn select_tile(
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
) {
    for e in event.read() {
        //let event_val = ulam::value_of_xy(e.0, e.1);
        for (mut location, parent_entity) in lands.iter_mut() {
            if location.x == e.0 && location.y == e.1 {
                if last_selected_tile.0 == e.0 && last_selected_tile.1 == e.1 && !location.selected
                {
                    // spawn
                    info!("spawn branch1");
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
                            );
                        });
                    location.selected = true;
                } else if last_selected_tile.0 != e.0
                    && last_selected_tile.1 != e.1
                    && !location.selected
                {
                    info!("last selected res set for {}, {}", e.0, e.1);
                    *last_selected_tile = LastSelectedTile(e.0, e.1);
                } else if location.selected {
                    for (sentity, slocation) in selected_lands.iter_mut() {
                        if slocation.x == e.0
                            && slocation.y == e.1
                            && slocation.x == location.x
                            && slocation.y == location.y
                        {
                            info!("despawn branch");
                            commands.entity(sentity).despawn();
                            location.selected = false;
                        }
                    }
                } else {
                    // info!("this shouldnt be reached");
                    // info!(
                    //     "{}, {}, {}, {}, {}, {}, {}",
                    //     e.0,
                    //     e.1,
                    //     location.x,
                    //     location.y,
                    //     last_selected_tile.0,
                    //     last_selected_tile.1,
                    //     location.selected
                    // );
                    *last_selected_tile = LastSelectedTile(e.0, e.1);
                }
            }
        }
    }
}

fn cancel_highlight_tile(
    mut commands: Commands,
    mut selected_lands: Query<
        (Entity, &Location),
        (With<Selected>, Without<Land>, Without<BuildingStructure>),
    >,
    mut event: EventReader<SelectTileEvent>,
) {
    for e in event.read() {}
}
