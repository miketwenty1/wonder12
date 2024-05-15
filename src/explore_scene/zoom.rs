use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    componenty::{BuildingStructure, TileText, ZoomInButton, ZoomOutButton},
    consty::{BUILDING_ZOOM_OUT_MAX, TEXT_ZOOM_OUT_MAX, ZOOM_IN_MAX, ZOOM_OUT_MAX},
    resourcey::{ColorPalette, ToggleMap},
    utils::distance_between_vecs,
};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn zoom_out_button_system(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>, With<ZoomOutButton>),
    >,
    time: Res<Time>,
    mut text_query: Query<&mut Text>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    //mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
    colors: Res<ColorPalette>,
) {
    let mut zoom_out = false;
    let mut zoom_amount: f32 = 0.0;

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // for mobile keep eye on https://github.com/bevyengine/bevy/pull/10930
                //clear_last_selected.send(ClearLastSelectedTile);
                mouse.clear(); //mouse.clear_just_pressed(MouseButton::Left);
                touches.clear();
                text.sections[0].value = "-".to_string();
                *color = colors.button_color.into();
                border_color.0 = colors.light_color;
                zoom_out = true;
                zoom_amount = 0.25;
            }
            Interaction::Hovered => {
                text.sections[0].value = "-".to_string();
                *color = colors.accent_color.into();
                border_color.0 = colors.node_color;
            }
            Interaction::None => {
                text.sections[0].value = "-".to_string();
                *color = colors.button_color.into();
                border_color.0 = colors.node_color;
            }
        }
    }

    for mouse_wheel in mouse_wheel_events.read() {
        if mouse_wheel.y < 0.0 {
            zoom_out = true;
            zoom_amount = 0.25 * time.delta_seconds() * 20.0;
        }
    }

    if zoom_out {
        for mut ortho in cam_query.iter_mut() {
            ortho.scale += zoom_amount;
            //info!("{}", ortho.scale);
            if ortho.scale > ZOOM_OUT_MAX {
                ortho.scale = ZOOM_OUT_MAX;
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn zoom_in_button_system(
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut touches: ResMut<Touches>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
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
    time: Res<Time>,
    mut text_query: Query<&mut Text>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    //mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
    colors: Res<ColorPalette>,
) {
    let mut zoom_in = false;
    let mut zoom_amount: f32 = 0.0;

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // for mobile keep eye on https://github.com/bevyengine/bevy/pull/10930
                //clear_last_selected.send(ClearLastSelectedTile);
                mouse.clear(); //.clear_just_pressed(MouseButton::Left);
                touches.clear();
                text.sections[0].value = "+".to_string();
                *color = colors.button_color.into();
                border_color.0 = colors.light_color;
                zoom_in = true;
                zoom_amount = 0.25;
            }
            Interaction::Hovered => {
                text.sections[0].value = "+".to_string();
                *color = colors.accent_color.into();
                border_color.0 = colors.node_color;
            }
            Interaction::None => {
                text.sections[0].value = "+".to_string();
                *color = colors.button_color.into();
                border_color.0 = colors.node_color;
            }
        }
    }

    for mouse_wheel in mouse_wheel_events.read() {
        if mouse_wheel.y > 0.0 {
            zoom_in = true;
            zoom_amount = 0.25 * time.delta_seconds() * 20.0;
        }
    }
    if zoom_in {
        for mut ortho in cam_query.iter_mut() {
            ortho.scale -= zoom_amount;
            if ortho.scale < ZOOM_IN_MAX {
                ortho.scale = ZOOM_IN_MAX;
            }
            //info!("{}", ortho.scale);
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn pinch_system(
    touches: Res<Touches>,
    mut touch_e: EventReader<TouchInput>,
    time: Res<Time>,
    mut cam_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut multitouch_distance: Local<f32>,
) {
    for _e in touch_e.read() {
        let mut zoom_amount: f32 = 0.0;
        if touches.iter().count() == 2 {
            let first = touches
                .first_pressed_position()
                .unwrap_or(Vec2 { x: 0.0, y: 0.0 });

            for touch in touches.iter() {
                if touch.position() != first {
                    let diff2 = distance_between_vecs(&touch.position(), &first);
                    if diff2 > 110.0 {
                        if (*multitouch_distance - diff2).abs() > 2.0 {
                            if *multitouch_distance == 0.0 {
                            } else if *multitouch_distance > diff2 {
                                zoom_amount = 0.25;
                                info!("zooming out");
                            } else {
                                zoom_amount = -0.25;
                                info!("zooming in");
                            }
                        }

                        *multitouch_distance = diff2;
                    }
                }
            }

            if zoom_amount != 0.0 {
                let time_adjusted = if time.delta_seconds() > 0.01 {
                    0.01
                } else {
                    time.delta_seconds()
                };
                for mut ortho in cam_query.iter_mut() {
                    ortho.scale += zoom_amount * time_adjusted * 30.0;
                    if ortho.scale > ZOOM_OUT_MAX {
                        ortho.scale = ZOOM_OUT_MAX;
                    } else if ortho.scale < ZOOM_IN_MAX {
                        ortho.scale = ZOOM_IN_MAX;
                    }
                }
            }
        } else {
            *multitouch_distance = 0.0;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn cam_ortho_scale_text_visibility(
    cam_query: Query<&OrthographicProjection, (With<Camera>, Changed<OrthographicProjection>)>,
    mut text_q: Query<&mut Visibility, With<TileText>>,
    mut building_q: Query<&mut Visibility, (With<BuildingStructure>, Without<TileText>)>,
    // mut land_q: Query<&mut Visibility, (With<Land>, Without<TileText>, Without<BuildingStructure>)>,
    toggle_map: Res<ToggleMap>,
) {
    let show_text_toggle = toggle_map.0.get("showtext").unwrap(); // true here means hide it, false means show it - no need to hide text if it's already hidden
    let show_building_toggle = toggle_map.0.get("showbuildings").unwrap();
    let cam = cam_query.get_single();
    if let Ok(ortho) = cam {
        // text
        if *show_text_toggle || ortho.scale >= TEXT_ZOOM_OUT_MAX {
            for mut text_visibility in text_q.iter_mut() {
                if *text_visibility == Visibility::Visible {
                    *text_visibility = Visibility::Hidden;
                }
            }
        } else if ortho.scale < TEXT_ZOOM_OUT_MAX {
            // despawn everything outside of camera view.
            for mut text_visibility in text_q.iter_mut() {
                if *text_visibility == Visibility::Hidden {
                    *text_visibility = Visibility::Visible;
                }
            }
        }

        // buildings
        if *show_building_toggle || ortho.scale >= BUILDING_ZOOM_OUT_MAX {
            for mut building_visibility in building_q.iter_mut() {
                if *building_visibility == Visibility::Visible {
                    *building_visibility = Visibility::Hidden;
                }
            }
        } else if ortho.scale < BUILDING_ZOOM_OUT_MAX {
            for mut building_visibility in building_q.iter_mut() {
                if *building_visibility == Visibility::Hidden {
                    *building_visibility = Visibility::Visible;
                }
            }
        }

        // // land
        // if ortho.scale >= BUILDING_ZOOM_OUT_MAX {
        //     info!("this working greater");
        //     for mut land_visibility in land_q.iter_mut() {
        //         info!("land visibility: {:?}", land_visibility);
        //         if *land_visibility == Visibility::Visible
        //             || *land_visibility == Visibility::Inherited
        //         {
        //             info!("GREATER land visibility: {:?}", land_visibility);
        //             *land_visibility = Visibility::Hidden;
        //         }
        //     }
        // } else if ortho.scale < BUILDING_ZOOM_OUT_MAX {
        //     info!("this working inner");
        //     for mut land_visibility in land_q.iter_mut() {
        //         info!("land visibility: {:?}", land_visibility);
        //         if *land_visibility == Visibility::Hidden {
        //             info!("INNER land visibility: {:?}", land_visibility);
        //             *land_visibility = Visibility::Visible;
        //         }
        //     }
        // }
    }
}
