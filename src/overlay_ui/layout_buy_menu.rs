use bevy::prelude::*;

use crate::{
    componenty::{
        BlockCostText, BlockHeightCartText, CartButton, CurrentBlockDateText,
        CurrentBlockLnAddressText, CurrentBlockMessageText, CurrentBlockOwnerText,
        CurrentBlockValueText, NewBlockColorButton, NewBlockColorText, NewBlockDataButton,
        NewBlockLnAddressButton, NewBlockLnAddressText, NewBlockMessageButton, NewBlockMessageText,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    keyboard::{components::KeyboardNode, KeyboardState},
    resourcey::{TileCart, TileCartData, TileCartVec},
    statey::ExploreState,
};

use super::UiOverlay;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

//const INTRO_TEXT: &str = "This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!";

#[derive(Component)]
pub struct ButtonGo;

#[derive(Component)]
pub struct ButtonBack;

pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    tile_cart: Res<TileCart>,
    mut tile_cart_vec: ResMut<TileCartVec>,
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

                background_color: BackgroundColor(Color::Rgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.5,
                    alpha: 0.15,
                }),
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
                    background_color: BackgroundColor(Color::PINK),
                    ..Default::default() // style: Style {
                })
                .with_children(|builder| {
                    spawn_new_total_cart_cost(
                        builder,
                        font.clone(),
                        &format!("Total: {} sats", cart_total),
                        20.0,
                        Color::WHITE,
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
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..Default::default()
                })
                .with_children(|builder| {
                    setup_left_block_menu_button(builder, font.clone(), "<-".to_string());
                    spawn_block_text_bundle(
                        builder,
                        font.clone(),
                        &format!("Block {}", tile_cart_vec.vec[0].height),
                        20.0,
                        Color::WHITE,
                    );
                    setup_right_block_menu_button(builder, font.clone(), "->".to_string());
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
                    background_color: BackgroundColor(Color::PINK),
                    ..Default::default() // style: Style {
                })
                .with_children(|builder| {
                    spawn_new_value_block_cost(
                        builder,
                        font.clone(),
                        &format!("Cost: {} sats", tile_cart_vec.vec[0].cost),
                        20.0,
                        Color::WHITE,
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
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                })
                .with_children(|builder| {
                    new_value_title(builder, font.clone(), "Set New Values", 20.0, Color::WHITE);
                    setup_ln_addr_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_LN_TEXT.to_string(),
                    );
                    setup_color_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_COLOR_TEXT.to_string(),
                    );
                    setup_message_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_MESSAGE_TEXT.to_string(),
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
                        padding: UiRect::all(Val::Px(1.0)),
                        //width: Val::Percent(100.0),
                        //height: Val::Percent(50.0),
                        //grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        //grid_template_rows: vec![GridTrack::auto()],
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..default()
                })
                .with_children(|builder| {
                    setup_config_for_all_blocks_button(builder, font.clone(), "X".to_string());
                    spawn_nested_text_bundle_flex(
                        builder,
                        font.clone(),
                        "Use these values for every blocks",
                        15.0,
                        Color::WHITE,
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
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                })
                .with_children(|builder| {
                    current_value_title(builder, font.clone(), "Current Data", 20.0, Color::WHITE);
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
                        Color::WHITE,
                    );
                    current_value_right(
                        builder,
                        font.clone(),
                        tile_cart_vec.vec[0].owner.to_string(),
                        tile_cart_vec.vec[0].ln_address.to_string(),
                        16.0,
                        Color::WHITE,
                    );
                    current_value_message(
                        builder,
                        font.clone(),
                        &tile_cart_vec.vec[0].message.to_string(),
                        16.0,
                        Color::WHITE,
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
                    setup_buy_create_invoice_button(builder, font.clone(), "Buy".to_string());
                    setup_back_from_buy_menu_button(builder, font.clone(), "Back".to_string());
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
    color: Color,
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
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color,
                },
            ));
        });
}

fn spawn_new_value_block_cost(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    color: Color,
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
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size,
                        color,
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
            background_color: BackgroundColor(Color::PURPLE),
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
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonBack,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 12.0,
                            color: Color::rgb(1., 1., 1.),
                        },
                    ));
                });
        });
}

fn setup_buy_create_invoice_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
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
            background_color: BackgroundColor(Color::PURPLE),

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
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonGo,
                ))
                .with_children(|parent2| {
                    parent2.spawn(TextBundle::from_section(
                        button_text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 16.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                        },
                    ));
                });
        });
}

fn setup_back_from_buy_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
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
            background_color: BackgroundColor(Color::RED),
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
                        background_color: NORMAL_BUTTON.into(),
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
                            color: Color::rgb(1.0, 1.0, 1.0),
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
    color: Color,
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
            background_color: BackgroundColor(Color::RED),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color,
                },
            ));
        });
}

fn setup_left_block_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
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
                background_color: NORMAL_BUTTON.into(),
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
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            ));
        });
}

fn setup_right_block_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
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
                background_color: NORMAL_BUTTON.into(),
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
                    color: Color::rgb(1.0, 1.0, 1.0),
                },
            ));
        });
}

fn current_value_title(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    color: Color,
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
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color,
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
    color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    format!("Value: {}", value),
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
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
                        color,
                    },
                ),
                CurrentBlockDateText,
            ));
        });
}

fn current_value_right(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    owner: String,
    ln_address: String,
    font_size: f32,
    color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::RED),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    format!("Owner: {}", owner),
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color,
                    },
                ),
                CurrentBlockOwnerText,
            ));
            builder.spawn((
                TextBundle::from_section(
                    ln_address.to_string(),
                    TextStyle {
                        font,
                        font_size,
                        color,
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
    color: Color,
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
            background_color: BackgroundColor(Color::PINK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size,
                        color,
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
    color: Color,
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
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size,
                    color,
                },
            ));
        });
}

fn setup_ln_addr_menu_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
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
            background_color: BackgroundColor(Color::MAROON),
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
                            min_width: Val::Px(194.0),
                            max_width: Val::Px(194.0),
                            min_height: Val::Px(32.0),
                            max_height: Val::Px(32.0),
                            //min_height: Val::Px(80.0),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    NewBlockLnAddressButton,
                    NewBlockDataButton,
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

fn setup_color_menu_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
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
            background_color: BackgroundColor(Color::ORANGE_RED),
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
                            min_width: Val::Px(194.0),
                            max_width: Val::Px(194.0),
                            min_height: Val::Px(32.0),
                            max_height: Val::Px(32.0),
                            //min_height: Val::Px(80.0),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    NewBlockColorButton,
                    NewBlockDataButton,
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

fn setup_message_menu_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
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
            background_color: BackgroundColor(Color::PURPLE),
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
                            min_width: Val::Px(220.0),
                            max_width: Val::Px(220.0),
                            min_height: Val::Px(100.0),
                            max_height: Val::Px(100.0),
                            margin: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    NewBlockMessageButton,
                    NewBlockDataButton,
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
