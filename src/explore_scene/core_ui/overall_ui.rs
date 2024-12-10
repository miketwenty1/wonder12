use super::components::{
    ExplorerUiNode, ExplorerUiNodeBottom, ExplorerUiNodeLeft, ExplorerUiNodeMiddle,
    ExplorerUiNodeRight, ExplorerUiNodeTop,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn ui_explorer(mut commands: Commands) {
    let mut parent = commands.spawn((
        Node {
            display: Display::Grid,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            grid_template_columns: vec![
                GridTrack::min_content(),
                GridTrack::flex(1.0),
                GridTrack::min_content(),
            ],
            grid_template_rows: vec![
                GridTrack::min_content(),
                GridTrack::flex(1.0),
                GridTrack::min_content(),
            ],
            ..default()
        },
        ExplorerUiNode,
    ));

    // top
    parent.with_children(|builder| {
        builder.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                grid_column: GridPlacement::span(3),
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Px(50.0),
                ..default()
            },
            Visibility::Hidden,
            ExplorerUiNodeTop,
        ));
    });

    // left
    parent.with_children(|builder| {
        builder.spawn((
            Node {
                display: Display::Grid,
                //height: Val::Percent(100.0),
                //width: Val::Px(150.0),
                ..default()
            },
            ExplorerUiNodeLeft,
        ));
    });

    // middle
    parent.with_children(|builder| {
        builder.spawn((
            Node {
                display: Display::Grid,
                //width: Val::Percent(100.0),
                //height: Val::Percent(100.0),
                //grid_column: GridPlacement::span(1),
                //grid_row: GridPlacement::span(1),
                ..default()
            },
            ExplorerUiNodeMiddle,
        ));
    });

    // right
    parent.with_children(|builder| {
        builder.spawn((
            Node {
                display: Display::Grid,
                //height: Val::Percent(100.0),
                ..default()
            },
            ExplorerUiNodeRight,
        ));
    });

    // bottom
    parent.with_children(|builder| {
        builder.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                grid_column: GridPlacement::span(3),
                //width: Val::Percent(100.0),
                ..default()
            },
            ExplorerUiNodeBottom,
        ));
    });
}
