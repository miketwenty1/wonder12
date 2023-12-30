use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::{
    componenty::{ZoomInButton, ZoomOutButton},
    consty::{ZOOM_IN_MAX, ZOOM_OUT_MAX},
    eventy::ClearLastSelectedTile,
    resourcey::ColorPalette,
};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn zoom_out_button_system(
    mut mouse: ResMut<Input<MouseButton>>,
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
    mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
    colors: Res<ColorPalette>,
) {
    let mut zoom_out = false;
    let mut zoom_amount: f32 = 0.0;

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // for mobile keep eye on https://github.com/bevyengine/bevy/pull/10930
                clear_last_selected.send(ClearLastSelectedTile);
                mouse.clear(); //mouse.clear_just_pressed(MouseButton::Left);
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
    mut mouse: ResMut<Input<MouseButton>>,
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
    mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
    colors: Res<ColorPalette>,
) {
    let mut zoom_in = false;
    let mut zoom_amount: f32 = 0.0;

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // for mobile keep eye on https://github.com/bevyengine/bevy/pull/10930
                clear_last_selected.send(ClearLastSelectedTile);
                mouse.clear(); //.clear_just_pressed(MouseButton::Left);
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
