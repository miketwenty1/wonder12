use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    consty::{MOVE_VELOCITY_FACTOR, TOTAL_TILE_SCALE_SIZE},
    eventy::{ClearLastSelectedTile, EdgeEvent, SelectTileEvent},
    resourcey::Edge,
};

use super::explore::set_camera_tile_bounds;

#[allow(clippy::too_many_arguments)]
pub fn touch_event_system(
    touches: Res<Touches>,
    mut camera: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    time: Res<Time>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut select_tile_event: EventWriter<SelectTileEvent>,
    mut clear_last_selected: EventWriter<ClearLastSelectedTile>,
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

            let direction = Vec3::new(-touch.delta().x, touch.delta().y, 0.0);

            let timefactor = if time.delta_seconds() > 0.01 {
                0.01
            } else {
                time.delta_seconds()
            };

            cam_transform.translation +=
                direction * timefactor * cam_ortho.scale * MOVE_VELOCITY_FACTOR * 5.0;

            set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);

            if touches.just_pressed(touch.id()) {
                //info!("send touch select");

                select_tile_event.send(SelectTileEvent(x, y));

                //*last_selected_tile = LastSelectedTile(x, y);
            }
            if direction.x.abs() > 0.1 || direction.y.abs() > 0.1 {
                clear_last_selected.send(ClearLastSelectedTile);
            }
            //info!("touch World coords: {}/{}", x, y);
        }
    }
}
