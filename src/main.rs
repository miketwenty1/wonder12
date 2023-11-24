use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        touch::Touch,
        touchpad::{TouchpadMagnify, TouchpadRotate},
    },
    prelude::*,
    text::{scale_value, Text2dBounds},
    ui::RelativeCursorPosition,
    winit::WinitSettings,
};
use rand::Rng;

// first_pressed_position
// get_pressed
// just_pressed
// just_released

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
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        //.insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                zoom_out_button_system,
                zoom_in_button_system,
                mouse_camera_system,
                touch_event_system,
            ), //, print_mouse_events_system, touch_event_system
        )
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn mouse_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if mouse.pressed(MouseButton::Middle) || mouse.pressed(MouseButton::Left) {
        for event in mouse_motion_events.read() {
            for mut transform in camera.iter_mut() {
                let direction = Vec3::new(-event.delta.x, event.delta.y, 0.0);
                transform.translation += direction * time.delta_seconds() * 3.0;
                if transform.translation.x < -2400.0 {
                    transform.translation.x = -2400.0
                }
                if transform.translation.x > 2400.0 {
                    transform.translation.x = 2400.0
                }
                if transform.translation.y > 2400.0 {
                    transform.translation.y = 2400.0
                }
                if transform.translation.y < -2400.0 {
                    transform.translation.y = -2400.0
                }
            }
        }
    }
}

fn touch_event_system(
    mut touch_events: EventReader<TouchInput>,
    touches: Res<Touches>,
    mut camera: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    for touch in touches.iter() {
        for mut transform in camera.iter_mut() {
            let direction = Vec3::new(-touch.delta().x, touch.delta().y, 0.0);
            transform.translation += direction * time.delta_seconds() * 16.0;
            if transform.translation.x < -2400.0 {
                transform.translation.x = -2400.0
            }
            if transform.translation.x > 2400.0 {
                transform.translation.x = 2400.0
            }
            if transform.translation.y > 2400.0 {
                transform.translation.y = 2400.0
            }
            if transform.translation.y < -2400.0 {
                transform.translation.y = -2400.0
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
                    if ortho.scale > 3.0 {
                        ortho.scale = 3.0;
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
) {
    // ui camera
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("spritesheet/background-extruded.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        32,
        47,
        Some(Vec2::new(2.0, 2.0)),
        None,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let slightly_smaller_text_style = TextStyle {
        font,
        font_size: 24.0,
        color: Color::WHITE,
    };

    for y in 1..50 {
        for x in 1..50 {
            let locationcoord = Location {
                x: x - 25,
                y: y - 25,
                ulam: ulam::value_of_xy(x - 25, y - 25),
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
                                (-100. * 25.0) + (x as f32 * 100.),
                                (-100. * 25.0) + (y as f32 * 100.),
                                0.,
                            ),
                            scale: Vec3::new(3.0, 3.0, 1.0),
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
                            scale: Vec3::new(0.3333, 0.3333, 1.0),
                            ..Default::default()
                        },
                        ..default()
                    });
                });
        }
    }

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
