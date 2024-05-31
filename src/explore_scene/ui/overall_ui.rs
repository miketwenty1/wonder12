use bevy::prelude::*;

use super::components::ExplorerUiNode;

#[allow(clippy::too_many_arguments)]
pub fn ui_explorer(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>,
    // initblocks: Res<InitBlockCount>,
    // colors: Res<ColorPalette>,
    // mut loading_init_block_text: ResMut<NextState<InitLoadingBlocksState>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),

                ..default()
            },
            ..default()
        },
        ExplorerUiNode,
    ));
}
