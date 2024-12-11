use crate::{building_config::utils::get_text_color, resourcey::ColorMapToggle};
use bevy::prelude::*;

use crate::{
    componenty::{BuildingStructure, Land, Location, TileText},
    consty::TEXT_ZOOM_OUT_MAX,
    eventy::{ToggleBuildings, ToggleColors, ToggleText},
    resourcey::{MapTileMode, SpriteSheetLand, ToggleMap, WorldOwnedTileMap},
    structy::TileTextType,
};

use super::blockchain_color::get_index_color;

#[allow(clippy::too_many_arguments)]
pub fn buildings_visibility_event(
    mut toggle: EventReader<ToggleBuildings>,
    mut buildings_q: Query<&mut Visibility, With<BuildingStructure>>,
    toggle_map: Res<ToggleMap>,
) {
    for _t in toggle.read() {
        for mut building_visi in buildings_q.iter_mut() {
            if *toggle_map.0.get("showbuildings").unwrap() {
                *building_visi = Visibility::Hidden;
            } else {
                *building_visi = Visibility::Visible;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn land_color_event(
    mut toggle: EventReader<ToggleColors>,
    mut land_q: Query<(&mut Sprite, &Location, &Children), With<Land>>,
    mut text_query: Query<&mut TextColor>,
    //toggle_map: Res<ToggleMap>,
    tile_res: Res<WorldOwnedTileMap>,
    land: Res<SpriteSheetLand>,
    map_mode: Res<MapTileMode>,
) {
    for _t in toggle.read() {
        for (mut sprite, loc, children) in land_q.iter_mut() {
            let tempnum;
            (tempnum, sprite.color) = get_index_color(&map_mode, &tile_res, &loc.ulam);

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = tempnum;
                let mut text_color = text_query.get_mut(children[0]).unwrap();
                if map_mode.0 == ColorMapToggle::LandTile {
                    atlas.layout = land.layout.clone();
                    **text_color = Srgba::WHITE.into();
                } else {
                    **text_color = get_text_color(&sprite.color);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn change_tile_text_event(
    mut toggle: EventReader<ToggleText>,
    mut text_q: Query<(&mut Text, &Location, &mut Visibility), With<TileText>>,
    tile_res: Res<WorldOwnedTileMap>,
    cam_query: Query<&mut OrthographicProjection, With<Camera>>,
) {
    for t in toggle.read() {
        let zoom_level = cam_query.get_single().unwrap().scale;

        for (mut text, loc, mut visibility) in text_q.iter_mut() {
            let a = tile_res.map.get(&loc.ulam);
            if let Some(val) = a {
                match t.0 {
                    TileTextType::Height => {
                        if zoom_level < TEXT_ZOOM_OUT_MAX && *visibility == Visibility::Hidden {
                            *visibility = Visibility::Visible;
                        }
                        **text = val.height.to_string();
                    }
                    TileTextType::Value => {
                        if zoom_level < TEXT_ZOOM_OUT_MAX && *visibility == Visibility::Hidden {
                            *visibility = Visibility::Visible;
                        }
                        **text = val.cost.to_string();
                    }
                    TileTextType::Blank => {
                        if *visibility == Visibility::Visible {
                            *visibility = Visibility::Hidden;
                        }
                    }
                };
            } else {
                match t.0 {
                    TileTextType::Height => {
                        if zoom_level < TEXT_ZOOM_OUT_MAX && *visibility == Visibility::Hidden {
                            *visibility = Visibility::Visible;
                        }
                        **text = loc.ulam.to_string();
                    }
                    TileTextType::Value => {
                        if zoom_level < TEXT_ZOOM_OUT_MAX && *visibility == Visibility::Visible {
                            *visibility = Visibility::Hidden;
                        }
                        **text = "".to_string();
                    }
                    TileTextType::Blank => {
                        if *visibility == Visibility::Visible {
                            *visibility = Visibility::Hidden;
                        }
                    }
                };
            }
        }
    }
}
