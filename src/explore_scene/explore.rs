use bevy::prelude::*;
use rand::Rng;
use ulam::Quad;

use crate::{
    componenty::{Land, Location},
    consty::{CHUNK_TILE_SPAN_COUNT, TILE_SCALE, TOTAL_TILE_SCALE_SIZE},
    eventy::SpriteSpawnEvent,
    resourcey::{ChunkManager, Edge, SpriteSheetBg, WorldOwnedTileMap},
    structy::SpawnDiffData,
};

pub fn init_explorer(mut sprite_spawn_event: EventWriter<SpriteSpawnEvent>) {
    sprite_spawn_event.send(SpriteSpawnEvent);
}

#[allow(clippy::too_many_arguments)]
pub fn spawn_block_sprites(
    mut sprite_spawn_event: EventReader<SpriteSpawnEvent>,
    mut commands: Commands,
    texture_atlas_handle_bg: Res<SpriteSheetBg>,
    edge: Res<Edge>,
    mut chunk_map: ResMut<ChunkManager>,
    tile_map: Res<WorldOwnedTileMap>,
) {
    for _event in sprite_spawn_event.read() {
        info!("is this triggering?");

        let middle_y = (edge.top.tile + edge.bottom.tile) / 2;
        let middle_x = (edge.left.tile + edge.right.tile) / 2;

        //info!("middle_y: {}, middle_x: {}", middle_y, middle_x);
        let spawn_diff = SpawnDiffData {
            xstart: middle_x - CHUNK_TILE_SPAN_COUNT * 4,
            xend: middle_x + CHUNK_TILE_SPAN_COUNT * 4,
            ystart: middle_y - CHUNK_TILE_SPAN_COUNT * 4,
            yend: middle_y + CHUNK_TILE_SPAN_COUNT * 4,
        };

        let mut land_sprite_index: usize;
        let mut color_for_sprites;
        let mut color_for_tile;
        // let mut tile_text = "".to_string();

        for x in spawn_diff.xstart..=spawn_diff.xend {
            for y in spawn_diff.ystart..=spawn_diff.yend {
                let ulam_i = ulam::value_of_xy(x, y);

                if 10000 >= ulam_i && !chunk_map.map.contains_key(&ulam_i) {
                    chunk_map.map.insert(ulam_i, true);

                    let mut locationcoord = Location {
                        x,
                        y,
                        ulam: ulam::value_of_xy(x, y),
                        quad: ulam::quad_of_xy(x, y),
                        selected: false,
                    };
                    if locationcoord.ulam == 1 {
                        locationcoord.quad = Quad::SouthEast
                    } else if locationcoord.quad == Quad::SouthEast {
                        locationcoord.quad = Quad::South
                    } else if locationcoord.quad == Quad::East
                        && ulam::quad_of_value(locationcoord.ulam - 1) == Quad::SouthEast
                    {
                        locationcoord.quad = Quad::SouthEast;
                    }

                    // writing this code to make tile_text populate correctly where it updates tiles correctly based on toggle.

                    let mut rng = rand::thread_rng();
                    //land_sprite_index = rng.gen_range(1..=11);
                    if tile_map.map.contains_key(&locationcoord.ulam) {
                        color_for_sprites = tile_map.map.get(&locationcoord.ulam).unwrap().color;
                        land_sprite_index =
                            tile_map.map.get(&locationcoord.ulam).unwrap().land_index;
                        color_for_tile = Color::Rgba {
                            red: 1.,
                            green: 1.,
                            blue: 1.,
                            alpha: 1.,
                        };
                    } else {
                        land_sprite_index = rng.gen_range(1..=11);
                        color_for_tile = Color::Rgba {
                            red: 0.2,
                            green: 0.2,
                            blue: 0.2,
                            alpha: 1.0,
                        };
                    }

                    let mut _cmd = commands.spawn((
                        SpriteSheetBundle {
                            atlas: TextureAtlas {
                                layout: texture_atlas_handle_bg.layout.clone(),
                                index: land_sprite_index,
                            },
                            sprite: Sprite {
                                color: color_for_tile,
                                ..Default::default()
                            },

                            transform: Transform {
                                translation: Vec3::new(
                                    TOTAL_TILE_SCALE_SIZE * x as f32,
                                    TOTAL_TILE_SCALE_SIZE * y as f32,
                                    0.,
                                ),
                                scale: Vec3::new(TILE_SCALE, TILE_SCALE, 1.0),
                                ..Default::default()
                            },
                            texture: texture_atlas_handle_bg.texture.clone(),
                            ..Default::default()
                        },
                        locationcoord,
                        Land,
                    ));
                }
            }
        }
    }
}
