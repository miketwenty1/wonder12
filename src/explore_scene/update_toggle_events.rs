use bevy::prelude::*;

use crate::{
    componenty::{BuildingStructure, Land, Location, TileText},
    eventy::{ToggleBuildings, ToggleColors, ToggleText},
    resourcey::{TileMap, ToggleMap},
    structy::TileTextType,
};

#[allow(clippy::too_many_arguments)]
pub fn buildings_visibility_event(
    mut edge_event: EventReader<ToggleBuildings>,
    mut buildings_q: Query<&mut Visibility, With<BuildingStructure>>,
    toggle_map: Res<ToggleMap>,
) {
    for _edge_e in edge_event.read() {
        for mut building_visi in buildings_q.iter_mut() {
            if *toggle_map.0.get("hidebuildings").unwrap() {
                *building_visi = Visibility::Visible;
            } else {
                *building_visi = Visibility::Hidden;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn land_color_event(
    mut toggle: EventReader<ToggleColors>,
    mut land_q: Query<(&mut TextureAtlasSprite, &Location), With<Land>>,
    toggle_map: Res<ToggleMap>,
    tile_res: Res<TileMap>,
) {
    for _t in toggle.read() {
        for (mut texture, loc) in land_q.iter_mut() {
            if *toggle_map.0.get("hidecolors").unwrap() {
                let a = tile_res.map.get(&loc.ulam);
                if let Some(val) = a {
                    texture.color = val.color;
                    texture.index = 0;
                }
            } else {
                let a = tile_res.map.get(&loc.ulam);
                if let Some(_val) = a {
                    texture.color = Color::Rgba {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                        alpha: 1.0,
                    };
                    texture.index = tile_res.map.get(&loc.ulam).unwrap().land_index;
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn change_tile_text_event(
    mut toggle: EventReader<ToggleText>,
    mut text_q: Query<(&mut Text, &Location), With<TileText>>,
    tile_res: Res<TileMap>,
) {
    for t in toggle.read() {
        for (mut text, loc) in text_q.iter_mut() {
            let a = tile_res.map.get(&loc.ulam);
            if let Some(val) = a {
                match t.0 {
                    TileTextType::Height => {
                        text.sections[0].value = val.height.to_string();
                    }
                    TileTextType::Value => {
                        text.sections[0].value = val.amount.to_string();
                    }
                    TileTextType::Blank => {
                        text.sections[0].value = "".to_string();
                    }
                };
            } else {
                match t.0 {
                    TileTextType::Height => {
                        text.sections[0].value = loc.ulam.to_string();
                    }
                    TileTextType::Value => {
                        text.sections[0].value = "".to_string();
                    }
                    TileTextType::Blank => {
                        text.sections[0].value = "".to_string();
                    }
                };
            }
        }
    }
}
// for (mut text, loc) in text_q.iter_mut() {
//     if *toggle_map.0.get("showvalues").unwrap() {
//         let a = tile_res.map.get(&loc.ulam);
//         if let Some(val) = a {
//             text.sections[0].value = val.amount.to_string();
//         }
//     } else {
//         let a = tile_res.map.get(&loc.ulam);
//         if let Some(_val) = a {
//             texture.color = Color::Rgba {
//                 red: 1.0,
//                 green: 1.0,
//                 blue: 1.0,
//                 alpha: 1.0,
//             };
//             texture.index = tile_res.map.get(&loc.ulam).unwrap().land_index;
//         }
//     }
// }
