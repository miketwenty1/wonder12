use bevy::{color::palettes::css::DARK_GREEN, prelude::*, utils::HashMap};

use crate::{
    consty::{CHUNK_PIXEL_SIZE, CHUNK_TILE_SPAN_COUNT},
    explore_scene::core_ui::paint_palette::resource::DefaultDrawColorPalette,
    resourcey::{ColorPalette, Edge, SpriteIndexBuilding, ToggleMap},
    structy::EdgeData,
};

pub fn init_hardcoded_res(mut commands: Commands) {
    let mut toggle_map = HashMap::new();
    toggle_map.insert("showbuildings".to_string(), false);
    toggle_map.insert("showcolors".to_string(), false);
    toggle_map.insert("showvalues".to_string(), true);
    toggle_map.insert("showheights".to_string(), false);
    toggle_map.insert("showtext".to_string(), false);

    commands.insert_resource(ToggleMap(toggle_map));

    let mut numbers_map = HashMap::new();
    numbers_map.insert(0, 0);
    numbers_map.insert(32, 1);
    numbers_map.insert(64, 1);
    numbers_map.insert(128, 2);
    numbers_map.insert(256, 3);
    numbers_map.insert(512, 4);
    numbers_map.insert(1024, 5);
    numbers_map.insert(2048, 6);
    numbers_map.insert(4096, 7);
    numbers_map.insert(8192, 8);
    numbers_map.insert(16384, 9);
    numbers_map.insert(32768, 10);
    numbers_map.insert(65536, 11);
    numbers_map.insert(131072, 11);
    numbers_map.insert(262144, 11);
    numbers_map.insert(524288, 11);
    numbers_map.insert(1048576, 11);
    numbers_map.insert(2097152, 11);
    numbers_map.insert(4194304, 11);
    numbers_map.insert(8388608, 11);

    commands.insert_resource(SpriteIndexBuilding(numbers_map));

    let color_palette = ColorPalette {
        node_color: Srgba::hex("222831").unwrap().into(),
        node_color_lighter: Srgba::hex("353d48").unwrap().into(),
        button_color: Srgba::hex("393E46").unwrap().into(),
        lite_button_color: Srgba::hex("6A7382").unwrap().into(),
        accent_color: Srgba::hex("00ADB5").unwrap().into(),
        light_color: Srgba::hex("EEEEEE").unwrap().into(),
        text_color: Srgba::hex("FAFAFA").unwrap().into(),
        red_color: Srgba::hex("B50800").unwrap().into(),
        yellow_color: Srgba::hex("ADB500").unwrap().into(),
        green_color: DARK_GREEN.into(),
    };

    commands.insert_resource(color_palette);

    let draw_palette = DefaultDrawColorPalette {
        colors: vec![
            // Absolute colors
            Srgba::hex("000000").unwrap().into(), // Black
            Srgba::hex("ffffff").unwrap().into(), // White
            // Grays and Silver
            Srgba::hex("808080").unwrap().into(), // Gray
            Srgba::hex("c0c0c0").unwrap().into(), // Silver
            // Reds and Pinks
            Srgba::hex("ff0000").unwrap().into(), // Red
            Srgba::hex("ff4500").unwrap().into(), // Orange Red
            Srgba::hex("dc143c").unwrap().into(), // Crimson
            Srgba::hex("8b0000").unwrap().into(), // Dark Red
            Srgba::hex("ff1493").unwrap().into(), // Deep Pink
            Srgba::hex("ff69b4").unwrap().into(), // Hot Pink
            Srgba::hex("ffc0cb").unwrap().into(), // Pink
            // Oranges
            Srgba::hex("ff8000").unwrap().into(), // Orange
            Srgba::hex("ffa500").unwrap().into(), // Dark Orange
            // Yellows
            Srgba::hex("ffff00").unwrap().into(), // Yellow
            Srgba::hex("ffd700").unwrap().into(), // Gold
            // Greens
            Srgba::hex("00ff00").unwrap().into(), // Green
            Srgba::hex("32cd32").unwrap().into(), // Lime Green
            Srgba::hex("006400").unwrap().into(), // Dark Green
            Srgba::hex("008000").unwrap().into(), // Dark Green
            Srgba::hex("80ff80").unwrap().into(), // Light Green
            // Cyans
            Srgba::hex("00ffff").unwrap().into(), // Cyan
            Srgba::hex("7fffd4").unwrap().into(), // Aquamarine
            Srgba::hex("66cdaa").unwrap().into(), // Medium Aquamarine
            Srgba::hex("20b2aa").unwrap().into(), // Light Sea Green
            Srgba::hex("008080").unwrap().into(), // Teal
            Srgba::hex("004040").unwrap().into(), // Dark Teal
            Srgba::hex("408080").unwrap().into(), // Light Teal
            Srgba::hex("80ffff").unwrap().into(), // Light Cyan
            // Blues
            Srgba::hex("0000ff").unwrap().into(), // Blue
            Srgba::hex("4682b4").unwrap().into(), // Steel Blue
            Srgba::hex("5f9ea0").unwrap().into(), // Cadet Blue
            Srgba::hex("000080").unwrap().into(), // Navy
            Srgba::hex("004080").unwrap().into(), // Darker Blue
            Srgba::hex("0080c0").unwrap().into(), // Sky Blue
            Srgba::hex("0080ff").unwrap().into(), // Bright Blue
            // Purples and Violets
            Srgba::hex("800080").unwrap().into(), // Purple
            Srgba::hex("4b0082").unwrap().into(), // Indigo
            Srgba::hex("6a5acd").unwrap().into(), // Slate Blue
            Srgba::hex("9370db").unwrap().into(), // Medium Purple
            Srgba::hex("8a2be2").unwrap().into(), // Blue Violet
            Srgba::hex("9400d3").unwrap().into(), // Dark Violet
            Srgba::hex("9932cc").unwrap().into(), // Dark Orchid
            Srgba::hex("ba55d3").unwrap().into(), // Medium Orchid
            Srgba::hex("ff00ff").unwrap().into(), // Magenta
            Srgba::hex("ff80ff").unwrap().into(), // Light Magenta
            // Browns
            Srgba::hex("a52a2a").unwrap().into(), // Brown
            Srgba::hex("d2691e").unwrap().into(), // Chocolate
            Srgba::hex("8b4513").unwrap().into(), // Saddle Brown
        ],
    };
    commands.insert_resource(draw_palette);

    // dividing by 2 to get middle locations.
    let start_edge = Edge {
        top: EdgeData {
            pixel: CHUNK_PIXEL_SIZE / 2.0,
            tile: CHUNK_TILE_SPAN_COUNT,
        },
        bottom: EdgeData {
            pixel: -CHUNK_PIXEL_SIZE / 2.0,
            tile: -CHUNK_TILE_SPAN_COUNT,
        },
        left: EdgeData {
            pixel: -CHUNK_PIXEL_SIZE / 2.0,
            tile: -CHUNK_TILE_SPAN_COUNT,
        },
        right: EdgeData {
            pixel: CHUNK_PIXEL_SIZE / 2.0,
            tile: CHUNK_TILE_SPAN_COUNT,
        },
    };
    commands.insert_resource(start_edge);
}
