use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{
    consty::{MOVE_VELOCITY_FACTOR, TILE_SCALE},
    eventy::{ClearLastSelectedTile, EdgeEvent},
    resourcey::{Edge, LastSelectedTile, MaxBlockHeight},
};

use super::explore::set_camera_tile_bounds;

pub fn clear_last_selected_tile(
    mut clear_tile_event: EventReader<ClearLastSelectedTile>,
    mut last_selected_tile: ResMut<LastSelectedTile>,
) {
    for _e in clear_tile_event.read() {
        *last_selected_tile = LastSelectedTile(1_000_000, 1_000_000);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn desktop_movement_camera_system(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse: Res<Input<MouseButton>>,
    mut q_camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
    keys: Res<Input<KeyCode>>,
    max_height: Res<MaxBlockHeight>,
    mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
) {
    for event in mouse_motion_events.read() {
        if mouse.pressed(MouseButton::Middle) || mouse.pressed(MouseButton::Left) {
            for (mut cam_transform, cam_ortho) in q_camera.iter_mut() {
                let direction = if ulam::value_of_xy(0, edge.bottom.tile) + 1_000 > max_height.0 {
                    Vec3::new(-event.delta.x, 100.0, 0.0)
                } else if ulam::value_of_xy(0, edge.top.tile) + 1_000 > max_height.0 {
                    Vec3::new(-event.delta.x, -100.0, 0.0)
                } else if ulam::value_of_xy(edge.left.tile, 0) + 1_000 > max_height.0 {
                    Vec3::new(100.0, event.delta.y, 0.0)
                } else if ulam::value_of_xy(edge.right.tile, 0) + 1_000 > max_height.0 {
                    Vec3::new(-100.0, event.delta.y, 0.0)
                } else {
                    Vec3::new(-event.delta.x, event.delta.y, 0.0)
                };

                if direction.x.abs() > 0.1 || direction.y.abs() > 0.1 {
                    clear_last_selected.send(ClearLastSelectedTile);
                }

                let timefactor = if time.delta_seconds() > 0.01 {
                    0.01
                } else {
                    time.delta_seconds()
                };

                let total_distance =
                    direction * timefactor * TILE_SCALE * cam_ortho.scale * MOVE_VELOCITY_FACTOR;

                let clamped_length = total_distance.clamp_length_max(300.0);

                cam_transform.translation += clamped_length;

                set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
            }
        }
    }
    if keys.pressed(KeyCode::W)
        || keys.pressed(KeyCode::A)
        || keys.pressed(KeyCode::S)
        || keys.pressed(KeyCode::D)
        || keys.pressed(KeyCode::Up)
        || keys.pressed(KeyCode::Left)
        || keys.pressed(KeyCode::Down)
        || keys.pressed(KeyCode::Right)
    {
        for (mut cam_transform, cam_ortho) in q_camera.iter_mut() {
            let y: f32 = if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
                15.0
            } else if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
                -15.0
            } else {
                0.0
            };
            let x: f32 = if keys.pressed(KeyCode::A) || keys.pressed(KeyCode::Left) {
                -15.0
            } else if keys.pressed(KeyCode::D) || keys.pressed(KeyCode::Right) {
                15.0
            } else {
                0.0
            };

            // push back on people trying to go too far
            let direction = if ulam::value_of_xy(0, edge.bottom.tile) + 1_000 > max_height.0 {
                Vec3::new(x, 100.0, 0.0)
            } else if ulam::value_of_xy(0, edge.top.tile) + 1_000 > max_height.0 {
                Vec3::new(x, -100.0, 0.0)
            } else if ulam::value_of_xy(edge.left.tile, 0) + 1_000 > max_height.0 {
                Vec3::new(100.0, y, 0.0)
            } else if ulam::value_of_xy(edge.right.tile, 0) + 1_000 > max_height.0 {
                Vec3::new(-100.0, y, 0.0)
            } else {
                Vec3::new(x, y, 0.0)
            };

            if direction.x.abs() > 0.1 || direction.y.abs() > 0.1 {
                clear_last_selected.send(ClearLastSelectedTile);
            }

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
