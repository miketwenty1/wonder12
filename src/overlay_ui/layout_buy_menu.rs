use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigButton, AllCartConfigText, BlockCostText, BlockHeightCartText, BuyMenuButton,
        CartButton, CouldBeEditabledTextBox, CurrentBlockDateText, CurrentBlockLnAddressText,
        CurrentBlockMessageText, CurrentBlockUsernameText, CurrentBlockValueText, EditabledTextBox,
        NewBlockColorButton, NewBlockColorText, NewBlockDataButton, NewBlockLnAddressButton,
        NewBlockLnAddressText, NewBlockMessageButton, NewBlockMessageText,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    keyboard::{components::KeyboardNode, KeyboardState},
    resourcey::{ColorPalette, TileCart, TileCartData, TileCartVec},
    statey::ExploreState,
};

use super::UiOverlay;

#[derive(Component)]
pub struct ButtonBack;

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    tile_cart: Res<TileCart>,
    mut tile_cart_vec: ResMut<TileCartVec>,
    colors: Res<ColorPalette>,
) {
    explore_state.set(ExploreState::Paused);

    // set the Cart data and sort it in the vec.
    let mut a: Vec<TileCartData> = tile_cart.map.values().cloned().collect();
    a.sort_by_key(|tile| tile.height);
    tile_cart_vec.vec = a;
    tile_cart_vec.index = 0;
    let cart_total: u32 = tile_cart_vec.vec.iter().map(|tile| tile.cost).sum();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    // width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::Stretch,
                    align_content: AlignContent::Stretch,
                    justify_items: JustifyItems::Stretch,
                    //align_items: AlignItems::Center,
                    grid_template_columns: vec![GridTrack::auto()],
                    grid_template_rows: vec![
                        GridTrack::min_content(),
                        GridTrack::auto(),
                        GridTrack::min_content(),
                        GridTrack::auto(),
                        GridTrack::min_content(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                        //GridTrack::flex(1.0),
                    ],

                    max_width: Val::Px(800.0),
                    min_width: Val::Percent(25.0),
                    ..default()
                },

                background_color: BackgroundColor(colors.node_color),
                ..default()
            },
            UiOverlay,
        ))
        .with_children(|builder| {
            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            // TOTAL for cart
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,

                        //padding: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..Default::default() // style: Style {
                })
                .with_children(|builder| {
                    spawn_new_total_cart_cost(
                        builder,
                        font.clone(),
                        &format!("Total: {} sats", cart_total),
                        20.0,
                        colors.text_color,
                        colors.node_color,
                    );
                });

            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            // block height left and right
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,

                        //padding: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..Default::default()
                })
                .with_children(|builder| {
                    setup_left_block_menu_button(
                        builder,
                        font.clone(),
                        colors.button_color,
                        colors.text_color,
                        "<-".to_string(),
                    );
                    spawn_block_text_bundle(
                        builder,
                        font.clone(),
                        &format!("Block {}", tile_cart_vec.vec[0].height),
                        20.0,
                        colors.text_color,
                    );
                    setup_right_block_menu_button(
                        builder,
                        font.clone(),
                        "->".to_string(),
                        colors.button_color,
                        colors.text_color,
                    );
                });
            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            // specific cost for the block in context
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Row,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        justify_items: JustifyItems::Center,

                        //padding: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..Default::default() // style: Style {
                })
                .with_children(|builder| {
                    spawn_new_value_block_cost(
                        builder,
                        font.clone(),
                        &format!("Cost: {} sats", tile_cart_vec.vec[0].cost),
                        20.0,
                        colors.text_color,
                        colors.node_color,
                    );
                });

            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            //new block data
            builder
                .spawn(NodeBundle {
                    style: Style {
                        //width: Val::Percent(100.0),
                        //height: Val::Px(100.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        //padding: UiRect::all(Val::Px(2.0)),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                        ],
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                })
                .with_children(|builder| {
                    new_value_title(
                        builder,
                        font.clone(),
                        "Set New Values",
                        20.0,
                        colors.node_color,
                        colors.text_color,
                    );
                    setup_ln_addr_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_LN_TEXT.to_string(),
                        colors.accent_color,
                        colors.button_color,
                    );
                    setup_color_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_COLOR_TEXT.to_string(),
                        colors.accent_color,
                        colors.button_color,
                    );
                    setup_message_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_MESSAGE_TEXT.to_string(),
                        colors.accent_color,
                        colors.button_color,
                    );
                });

            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            // config
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        //justify_items: JustifyItems::Stretch,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::all(Val::Px(3.0)),
                        //margin: UiRect::all(Val::Px(3.0)),
                        //width: Val::Percent(100.0),
                        //height: Val::Percent(50.0),
                        //grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        //grid_template_rows: vec![GridTrack::auto()],
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                })
                .with_children(|builder| {
                    setup_config_for_all_blocks_button(
                        builder,
                        font.clone(),
                        " ".to_string(),
                        colors.accent_color,
                        colors.text_color,
                        colors.button_color,
                    );
                    spawn_nested_text_bundle_flex(
                        builder,
                        font.clone(),
                        "Use these values for every blocks",
                        15.0,
                        colors.node_color,
                        colors.text_color,
                    );
                });
            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            // current data
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        margin: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(2.0)),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                        ],
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                })
                .with_children(|builder| {
                    current_value_title(
                        builder,
                        font.clone(),
                        "Current Data",
                        20.0,
                        colors.node_color,
                        colors.text_color,
                    );
                    let datetime_string = tile_cart_vec.vec[0]
                        .event_date
                        .map_or("".to_string(), |datetime| {
                            datetime.date_naive().format("%Y-%m-%d").to_string()
                        });
                    current_value_left(
                        builder,
                        font.clone(),
                        (tile_cart_vec.vec[0].value).to_string(),
                        datetime_string,
                        16.0,
                        colors.node_color,
                        colors.text_color,
                    );
                    current_value_right(
                        builder,
                        font.clone(),
                        tile_cart_vec.vec[0].username.to_string(),
                        tile_cart_vec.vec[0].ln_address.to_string(),
                        16.0,
                        colors.text_color,
                        colors.node_color,
                    );
                    current_value_message(
                        builder,
                        font.clone(),
                        &tile_cart_vec.vec[0].message.to_string(),
                        16.0,
                        colors.node_color,
                        colors.text_color,
                    );
                });

            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            //buy and back buttons
            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        //width: Val::Percent(100.0),
                        //height: Val::Px(100.0),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        grid_template_rows: vec![GridTrack::auto()],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..default()
                })
                .with_children(|builder| {
                    setup_buy_create_invoice_button(
                        builder,
                        font.clone(),
                        "Buy".to_string(),
                        colors.node_color,
                        colors.button_color,
                        colors.text_color,
                    );
                    setup_back_from_buy_menu_button(
                        builder,
                        font.clone(),
                        "Back".to_string(),
                        colors.node_color,
                        colors.button_color,
                        colors.text_color,
                    );
                });
            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            //keyboard
            builder.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(23.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        //padding: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                },
                KeyboardNode,
            ));
        });
}

pub fn set_keyboard(mut buy_state: ResMut<NextState<KeyboardState>>) {
    buy_state.set(KeyboardState::On);
}

fn spawn_new_total_cart_cost(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    text_color: Color,
    node_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Start,
                align_content: AlignContent::Start,
                grid_column: GridPlacement::span(2),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color: text_color,
                },
            ));
        });
}

fn spawn_new_value_block_cost(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    text_color: Color,
    node_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Start,
                align_content: AlignContent::Start,
                grid_column: GridPlacement::span(2),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size,
                        color: text_color,
                    },
                ),
                BlockCostText,
            ));
        });
}
fn setup_config_for_all_blocks_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    text_color: Color,
    button_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                //flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(20.0),
                            height: Val::Px(20.0),
                            //justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    AllCartConfigButton,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        TextBundle::from_section(
                            button_text,
                            TextStyle {
                                font: font.clone(),
                                font_size: 12.0,
                                color: text_color,
                            },
                        ),
                        AllCartConfigText,
                    ));
                });
        });
}

fn setup_buy_create_invoice_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::End,
                align_items: AlignItems::Center,
                align_content: AlignContent::End,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),

            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    BuyMenuButton,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: text_color,
                        },
                    ));
                });
        });
}

fn setup_back_from_buy_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Center,
                align_content: AlignContent::Start,

                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(80.0),
                            height: Val::Px(30.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    ButtonBack,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: text_color,
                        },
                    ));
                });
        });
}

fn spawn_block_text_bundle(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    color: Color,
) {
    builder.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size,
                color,
            },
        ),
        BlockHeightCartText,
    ));
}

fn spawn_nested_text_bundle_flex(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    node_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                //justify_items: JustifyItems::Center,
                //flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color: text_color,
                },
            ));
        });
}

fn setup_left_block_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_color: Color,
    text_color: Color,
    button_text: String,
) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(25.0),
                    justify_items: JustifyItems::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(2.),
                        bottom: Val::Px(2.),
                    },
                    ..default()
                },
                background_color: button_color.into(),
                ..default()
            },
            CartButton(-1),
        ))
        .with_children(|parent2| {
            parent2.spawn(TextBundle::from_section(
                button_text,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: text_color,
                },
            ));
        });
}

fn setup_right_block_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    button_color: Color,
    text_color: Color,
) {
    builder
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(40.0),
                    height: Val::Px(25.0),
                    justify_items: JustifyItems::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    margin: UiRect {
                        left: Val::Px(10.),
                        right: Val::Px(10.),
                        top: Val::Px(2.),
                        bottom: Val::Px(2.),
                    },
                    ..default()
                },
                background_color: button_color.into(),
                ..default()
            },
            CartButton(1),
        ))
        .with_children(|parent2| {
            parent2.spawn(TextBundle::from_section(
                button_text,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: text_color,
                },
            ));
        });
}

fn current_value_title(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    node_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                grid_column: GridPlacement::span(2),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color: text_color,
                },
            ));
        });
}

fn current_value_left(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    value: String,
    event_date: String,
    font_size: f32,
    node_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    format!("Value: {} sats", value),
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color: text_color,
                    },
                ),
                CurrentBlockValueText,
            ));
            builder.spawn((
                TextBundle::from_section(
                    format!("Date: {}", event_date),
                    TextStyle {
                        font,
                        font_size,
                        color: text_color,
                    },
                ),
                CurrentBlockDateText,
            ));
        });
}

fn current_value_right(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    username: String,
    ln_address: String,
    font_size: f32,
    text_color: Color,
    node_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    format!("Owner: {}", username),
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color: text_color,
                    },
                ),
                CurrentBlockUsernameText,
            ));
            builder.spawn((
                TextBundle::from_section(
                    ln_address.to_string(),
                    TextStyle {
                        font,
                        font_size,
                        color: text_color,
                    },
                ),
                CurrentBlockLnAddressText,
            ));
        });
}

fn current_value_message(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    node_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Start,
                align_content: AlignContent::Start,
                grid_column: GridPlacement::span(2),
                padding: UiRect::all(Val::Px(2.0)),
                max_width: Val::Px(250.0),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size,
                        color: text_color,
                    },
                ),
                CurrentBlockMessageText,
            ));
        });
}

fn new_value_title(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    node_color: Color,
    text_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                grid_column: GridPlacement::span(2),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color: text_color,
                },
            ));
        });
}

fn setup_ln_addr_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                align_content: AlignContent::Center,
                //max_height: Val::Percent(50.0),
                //min_height: Val::Percent(50.0),
                min_width: Val::Px(198.0),
                max_width: Val::Px(198.0),
                min_height: Val::Px(36.0),
                max_height: Val::Px(36.0),
                //padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            align_content: AlignContent::Center,
                            margin: UiRect::all(Val::Px(1.0)),
                            flex_wrap: FlexWrap::Wrap,
                            min_width: Val::Px(196.0),
                            max_width: Val::Px(196.0),
                            min_height: Val::Px(34.0),
                            max_height: Val::Px(34.0),
                            //min_height: Val::Px(80.0),
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    NewBlockLnAddressButton,
                    NewBlockDataButton,
                    //EditabledTextBox,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        TextBundle::from_section(
                            button_text.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 16.0,
                                color: DEFAULT_NO_PICK_COLOR,
                            },
                        ),
                        NewBlockLnAddressText,
                    ));
                });
        });
}

fn setup_color_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                align_content: AlignContent::Center,
                //max_height: Val::Percent(50.0),
                //min_height: Val::Percent(50.0),
                min_width: Val::Px(198.0),
                max_width: Val::Px(198.0),
                min_height: Val::Px(36.0),
                max_height: Val::Px(36.0),
                //padding: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            align_content: AlignContent::Center,
                            margin: UiRect::all(Val::Px(1.0)),
                            //flex_wrap: FlexWrap::Wrap,
                            min_width: Val::Px(196.0),
                            max_width: Val::Px(196.0),
                            min_height: Val::Px(34.0),
                            max_height: Val::Px(34.0),
                            //min_height: Val::Px(80.0),
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    NewBlockColorButton,
                    NewBlockDataButton,
                    EditabledTextBox,
                    CouldBeEditabledTextBox,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        TextBundle::from_section(
                            button_text.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 16.0,
                                color: DEFAULT_NO_PICK_COLOR,
                            },
                        ),
                        NewBlockColorText,
                    ));
                });
        });
}

fn setup_message_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                justify_items: JustifyItems::Center,
                align_content: AlignContent::Center,
                grid_column: GridPlacement::span(2),
                min_width: Val::Px(224.0),
                max_width: Val::Px(224.0),
                min_height: Val::Px(104.0),
                max_height: Val::Px(104.0),
                //padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Start,
                            justify_items: JustifyItems::Start,
                            align_content: AlignContent::Start,
                            min_width: Val::Px(222.0),
                            max_width: Val::Px(222.0),
                            min_height: Val::Px(102.0),
                            max_height: Val::Px(102.0),
                            margin: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: button_color.into(),
                        ..default()
                    },
                    NewBlockMessageButton,
                    NewBlockDataButton,
                    EditabledTextBox,
                    CouldBeEditabledTextBox,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        TextBundle::from_section(
                            button_text.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 16.0,
                                color: DEFAULT_NO_PICK_COLOR,
                            },
                        ),
                        NewBlockMessageText,
                    ));
                });
        });
}
