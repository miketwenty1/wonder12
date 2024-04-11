use bevy::prelude::*;
use ulam::calc_coord::calc_xy;

use crate::{
    consty::{CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT, TOTAL_TILE_SCALE_SIZE},
    eventy::{EdgeEvent, TravelHeight},
    resourcey::Edge,
    structy::{EdgeData, EdgeType},
};

#[allow(clippy::type_complexity)]
pub fn travel_event(
    mut event: EventReader<TravelHeight>,
    mut q_camera: Query<&mut Transform, With<Camera>>,
    mut edge: ResMut<Edge>,
    mut edge_event: EventWriter<EdgeEvent>,
) {
    for height in event.read() {
        for mut cam_transform in q_camera.iter_mut() {
            let (x, y) = calc_xy(height.0);
            let x_trans = x as f32 * TOTAL_TILE_SCALE_SIZE;
            let y_trans = y as f32 * TOTAL_TILE_SCALE_SIZE;

            let new_edge = Edge {
                top: EdgeData {
                    pixel: y_trans + CHUNK_PIXEL_SIZE / 2.,
                    tile: y + CHUNK_TILE_SPAN_COUNT,
                },
                bottom: EdgeData {
                    pixel: y_trans - CHUNK_PIXEL_SIZE / 2.,
                    tile: y - CHUNK_TILE_SPAN_COUNT,
                },
                left: EdgeData {
                    pixel: x_trans - CHUNK_PIXEL_SIZE / 2.,
                    tile: x - CHUNK_TILE_SPAN_COUNT,
                },
                right: EdgeData {
                    pixel: x_trans + CHUNK_PIXEL_SIZE / 2.,
                    tile: x + CHUNK_TILE_SPAN_COUNT,
                },
            };
            *edge = new_edge;

            cam_transform.translation.x = x_trans;
            cam_transform.translation.y = y_trans;
            //set_camera_tile_bounds(cam_transform.translation, &mut edge, &mut edge_event);
            edge_event.send(EdgeEvent {
                edge_type: EdgeType::Bottom,
                x,
                y,
            });
        }
    }
}
