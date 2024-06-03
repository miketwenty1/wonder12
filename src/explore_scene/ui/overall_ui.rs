use super::components::{
    ExplorerUiNode, ExplorerUiNodeBottom, ExplorerUiNodeLeft, ExplorerUiNodeMiddle,
    ExplorerUiNodeRight, ExplorerUiNodeTop,
};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn ui_explorer(mut commands: Commands) {
    let mut parent = commands.spawn((
        NodeBundle {
            style: Style {
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
            //background_color: BackgroundColor(Color::ORANGE),
            ..default()
        },
        ExplorerUiNode,
    ));

    // top
    parent.with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    grid_column: GridPlacement::span(3),
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Px(50.0),
                    ..default()
                },
                //visibility: Visibility::Hidden,
                // background_color: BackgroundColor(Color::Rgba {
                //     red: 0.1,
                //     green: 0.1,
                //     blue: 0.1,
                //     alpha: 0.3,
                // }),
                ..default()
            },
            ExplorerUiNodeTop,
        ));
    });

    // left
    parent.with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    //height: Val::Percent(100.0),
                    //width: Val::Px(150.0),
                    ..default()
                },
                // background_color: BackgroundColor(Color::Rgba {
                //     red: 0.1,
                //     green: 0.1,
                //     blue: 0.1,
                //     alpha: 0.3,
                // }),
                //background_color: BackgroundColor(Color::RED),
                ..default()
            },
            ExplorerUiNodeLeft,
        ));
    });

    // middle
    parent.with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    //width: Val::Percent(100.0),
                    //height: Val::Percent(100.0),
                    //grid_column: GridPlacement::span(1),
                    //grid_row: GridPlacement::span(1),
                    ..default()
                },
                //background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.5)),
                ..default()
            },
            ExplorerUiNodeMiddle,
        ));
    });

    // right
    parent.with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    //height: Val::Percent(100.0),
                    ..default()
                },
                // background_color: BackgroundColor(Color::Rgba {
                //     red: 0.1,
                //     green: 0.1,
                //     blue: 0.1,
                //     alpha: 0.3,
                // }),
                ..default()
            },
            ExplorerUiNodeRight,
        ));
    });

    // bottom
    parent.with_children(|builder| {
        builder.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    grid_column: GridPlacement::span(3),
                    //width: Val::Percent(100.0),
                    ..default()
                },
                // background_color: BackgroundColor(Color::Rgba {
                //     red: 0.1,
                //     green: 0.1,
                //     blue: 0.1,
                //     alpha: 0.3,
                // }),
                ..default()
            },
            ExplorerUiNodeBottom,
        ));
    });
}
