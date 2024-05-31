use bevy::prelude::*;

use crate::componenty::DrawOverlayMesh;

pub fn cleanup_system(
    mut commands: Commands,
    mut mesh_q: Query<&mut Visibility, With<DrawOverlayMesh>>,
) {
    info!("hide em");
    for mut q in mesh_q.iter_mut() {
        info!("hide em!");
        *q = Visibility::Hidden;
    }
}
