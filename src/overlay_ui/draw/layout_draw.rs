use bevy::prelude::*;

use crate::{componenty::DrawOverlayMesh, resourcey::ColorPalette};

use super::DrawScene;

#[allow(clippy::too_many_arguments)]
pub fn spawn_layout(
    mut commands: Commands,
    mut mesh_q: Query<&mut Visibility, With<DrawOverlayMesh>>,
    colors: Res<ColorPalette>,
) {
    for mut q in mesh_q.iter_mut() {
        *q = Visibility::Visible;
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Stretch,
                    align_content: AlignContent::Stretch,
                    justify_items: JustifyItems::Stretch,
                    //align_items: AlignItems::Center,
                    grid_template_columns: vec![GridTrack::auto()],
                    grid_template_rows: vec![
                        GridTrack::auto(), // total
                        GridTrack::auto(), // <- block ->
                        GridTrack::auto(), // cost
                        GridTrack::auto(), // set new values
                        GridTrack::auto(), // config box
                        GridTrack::auto(), // current messages
                        GridTrack::auto(), // buy / back
                        GridTrack::auto(), // keyboard
                    ],
                    ..default()
                },

                background_color: BackgroundColor(colors.node_color),
                ..default()
            },
            DrawScene,
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            });
        });
}
