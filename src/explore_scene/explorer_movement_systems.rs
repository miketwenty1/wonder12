use bevy::prelude::*;

use crate::{eventy::ClearLastSelectedTile, resourcey::LastSelectedTile};

pub fn clear_last_selected_tile(
    mut clear_tile_event: EventReader<ClearLastSelectedTile>,
    mut last_selected_tile: ResMut<LastSelectedTile>,
) {
    for _e in clear_tile_event.read() {
        *last_selected_tile = LastSelectedTile(1_000_000, 1_000_000);
    }
}
