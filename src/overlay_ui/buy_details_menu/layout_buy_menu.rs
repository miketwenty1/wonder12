use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigButton, AllCartConfigText, BlockCostText, BlockHeightCartText,
        BlockUiMessageItem, BtnShowingColor, BuyMenuButton, CartButton, CouldBeEditabledTextBox,
        CurrentBlockMessageNode, EditabledTextBox, HideMessageBtn, NewBlockColorButton,
        NewBlockColorText, NewBlockDataButton, NewBlockLnAddressButton, NewBlockLnAddressText,
        NewBlockMessageButton, NewBlockMessageText,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    eventy::{BlockDetailMessage, KeyboardSpawnEvent},
    keyboard::{components::KeyboardNode, KeyboardState},
    resourcey::{
        ColorPalette, CurrentCartBlock, TileCart, TileCartData, TileCartVec, User,
        UserPurchasedBlockMessage, WinSize,
    },
    statey::{ExploreSelectState, ExploreState},
};

use super::BuyDetailsMenu;

#[derive(Component)]
pub struct ButtonBack;

#[allow(clippy::too_many_arguments)]
pub fn spawn_layout(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    mut game_select_set_state: ResMut<NextState<ExploreSelectState>>,
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
    tile_cart: Res<TileCart>,
    mut current_cart_item: ResMut<CurrentCartBlock>,
    mut tile_cart_vec: ResMut<TileCartVec>,
    colors: Res<ColorPalette>,
    win: Res<WinSize>,
    mut keyboard_event: EventWriter<KeyboardSpawnEvent>,
    user: Res<User>,
    mut message_writer: EventWriter<BlockDetailMessage>,
) {
    //info!("current_cart_item {:#?}", current_cart_item);
    explore_state.set(ExploreState::Paused);
    game_select_set_state.set(ExploreSelectState::Off);
    keyboard_state.set(KeyboardState::Off);

    // set the Cart data and sort it in the vec.
    let mut a: Vec<TileCartData> = tile_cart.map.values().cloned().collect();
    a.sort_by_key(|tile| tile.height);
    tile_cart_vec.vec = a;
    tile_cart_vec.index = 0;
    let cart_total: u32 = tile_cart_vec.vec.iter().map(|tile| tile.cost).sum();
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let first_cart_block = tile_cart_vec.vec.first().unwrap();
    // if there might be messages (i.e. value > 128, meaning more than just the current potential message then we'll query db)
    if first_cart_block.value > 128 {
        message_writer.send(BlockDetailMessage(first_cart_block.height));
    }

    // setting init color to show on buy screen square
    current_cart_item.color = tile_cart_vec.vec[0].new_color;

    let multi_select_visi = if tile_cart_vec.vec.len() > 1 {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    let w_size = if win.width > 450.0 { 450.0 } else { win.width };
    let font_size_headings = if win.width < 420.0 { 16.0 } else { 24.0 };
    let font_size_text = if win.width < 420.0 { 14.0 } else { 16.0 };
    let edit_box_width = if win.width < 420.0 {
        (win.width / 2.0) - 1.0
    } else {
        210.0
    };
    let lightning_text = if user.ln_address.len() > 4 {
        &user.ln_address
    } else {
        DEFAULT_NEW_LN_TEXT
    };
    info!("spawning width of {}", w_size);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    // width: Val::Percent(100.0),
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

                    max_width: Val::Px(w_size), //Val::Px(800.0),
                    min_width: Val::Px(w_size), //Val::Percent(25.0),
                    ..default()
                },

                background_color: BackgroundColor(colors.node_color),
                ..default()
            },
            BuyDetailsMenu,
        ))
        .with_children(|builder| {
            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            // TOTAL for cart
            let mut toprow = builder.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,

                    //align_content: AlignContent::Stretch,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    //justify_items: JustifyItems::Stretch,

                    //padding: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(colors.node_color),
                ..Default::default() // style: Style {
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                spawn_new_total_cart_cost(
                    builder,
                    font.clone(),
                    &format!("Total: {} sats", cart_total),
                    font_size_headings,
                    colors.text_color,
                    colors.node_color,
                );
            });
            toprow.with_children(|builder| {
                builder.spawn(NodeBundle {
                    ..Default::default()
                });
            });
            toprow.with_children(|builder| {
                builder
                    .spawn(NodeBundle { ..default() })
                    .with_children(|innerbuilder| {
                        innerbuilder
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(30.0),
                                        height: Val::Px(30.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        align_content: AlignContent::Center,
                                        justify_items: JustifyItems::Center,
                                        ..default()
                                    },
                                    background_color: colors.red_color.into(),
                                    ..default()
                                },
                                ButtonBack,
                            ))
                            .with_children(|ccbuilder| {
                                ccbuilder.spawn(TextBundle::from_section(
                                    "X",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: colors.text_color,
                                    },
                                ));
                            });
                    });
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
                        multi_select_visi,
                    );
                    spawn_block_text_bundle(
                        builder,
                        font.clone(),
                        &format!("Block {}", tile_cart_vec.vec[0].height),
                        font_size_headings,
                        colors.text_color,
                    );
                    setup_right_block_menu_button(
                        builder,
                        font.clone(),
                        "->".to_string(),
                        colors.button_color,
                        colors.text_color,
                        multi_select_visi,
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
                        font_size_headings,
                        colors.text_color,
                        colors.node_color,
                    );
                });

            // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // // //
            //new block data
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        //height: Val::Px(100.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(2.0)),
                        //padding: UiRect::all(Val::Px(2.0)),
                        grid_template_columns: vec![GridTrack::auto(), GridTrack::auto()],
                        grid_template_rows: vec![
                            GridTrack::auto(),
                            GridTrack::auto(),
                            GridTrack::auto(),
                        ],
                        ..default()
                    },
                    //background_color: BackgroundColor(colors.node_color),
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                })
                .with_children(|builder| {
                    new_value_title(
                        builder,
                        font.clone(),
                        "Set New Values",
                        font_size_headings,
                        colors.node_color,
                        colors.text_color,
                        current_cart_item.color,
                    );
                    setup_ln_addr_menu_button(
                        builder,
                        font.clone(),
                        lightning_text.to_string(),
                        colors.accent_color,
                        colors.button_color,
                        font_size_text,
                        edit_box_width,
                    );
                    setup_color_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_COLOR_TEXT.to_string(),
                        colors.accent_color,
                        colors.button_color,
                        font_size_text,
                        edit_box_width,
                    );
                    setup_message_menu_button(
                        builder,
                        font.clone(),
                        DEFAULT_NEW_MESSAGE_TEXT.to_string(),
                        colors.accent_color,
                        colors.button_color,
                        font_size_text,
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
                    visibility: multi_select_visi,
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
                        "Apply to all selected blocks",
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
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        // grid_template_columns: vec![GridTrack::auto()],
                        // grid_template_rows: vec![GridTrack::auto()],
                        ..default()
                    },
                    //border_color: BorderColor(colors.accent_color),
                    //colors.node_color),
                    ..default()
                })
                .with_children(|builder| {
                    current_message_title(
                        builder,
                        font.clone(),
                        "Block Log",
                        20.0,
                        colors.node_color,
                        colors.text_color,
                    );
                    let mut message_node = builder.spawn((
                        NodeBundle {
                            style: Style {
                                display: Display::Grid,
                                justify_items: JustifyItems::Center,
                                padding: UiRect::all(Val::Px(6.0)),
                                grid_template_columns: vec![
                                    GridTrack::auto(),
                                    GridTrack::auto(),
                                    GridTrack::auto(),
                                ],
                                // purposely leaving this commented out because i'll be adding various amounts of rows.
                                //grid_template_rows: vec![GridTrack::auto()],
                                ..default()
                            },
                            background_color: BackgroundColor(colors.node_color_lighter),
                            //background_color: BackgroundColor(Color::GREEN), //colors.node_color),
                            ..default()
                        },
                        CurrentBlockMessageNode,
                    ));
                    message_node.with_children(|builder| {
                        match &tile_cart_vec.vec[tile_cart_vec.index].messages {
                            Some(s) => {
                                spawn_messages(
                                    builder,
                                    font.clone(),
                                    s.to_vec(),
                                    16.0,
                                    colors.clone(),
                                );
                            }
                            None => {}
                        };
                    });
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
                        colors.green_color, //button color
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
                    //background_color: BackgroundColor(Color::BLUE),
                    ..default()
                },
                KeyboardNode,
            ));
        });
    keyboard_event.send(KeyboardSpawnEvent);
}

// pub fn set_keyboard(
//     mut state: ResMut<NextState<KeyboardState>>,
//     mut keyboard_event: EventReader<KeyboardSpawnEvent>,
// ) {
//     for _e in keyboard_event.read() {
//         info!("BEFORE: event received keyboard state is {:?}", state);
//         state.set(KeyboardState::On);
//         info!("NOW: event received keyboard state is {:?}", state);
//     }
// }

fn spawn_new_total_cart_cost(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    text_color: Color,
    _node_color: Color,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                // display: Display::Flex,
                // justify_items: JustifyItems::Center,
                // align_items: AlignItems::Center,
                // align_content: AlignContent::Center,
                // grid_column: GridPlacement::span(1),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            //background_color: BackgroundColor(node_color),
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
    visibility: Visibility,
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
                visibility,
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
    visibility: Visibility,
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
                visibility,
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

fn current_message_title(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    _text: &str,
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
                //grid_column: GridPlacement::span(3),
                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(node_color),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn((
                    ButtonBundle {
                        style: Style {
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: BackgroundColor(node_color), //node_color
                        ..default()
                    },
                    HideMessageBtn,
                ))
                .with_children(|innerbuilder| {
                    innerbuilder.spawn((TextBundle::from_section(
                        "Msg Toggle",
                        TextStyle {
                            font: font.clone(),
                            font_size,
                            color: text_color,
                        },
                    ),));
                });
        });
}

pub fn spawn_messages(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    messages: Vec<UserPurchasedBlockMessage>,
    font_size: f32,
    colors: ColorPalette,
) {
    for message in messages {
        builder
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        margin: UiRect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            top: Val::Px(1.0),
                            bottom: Val::Px(10.0),
                        },
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    //background_color: BackgroundColor(colors.node_color),
                    ..default()
                },
                BlockUiMessageItem,
            ))
            .with_children(|innerc| {
                innerc.spawn(TextBundle::from_section(
                    message.value.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 18.0,
                        color: colors.accent_color,
                    },
                ));
            });
        builder
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        margin: UiRect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            top: Val::Px(1.0),
                            bottom: Val::Px(10.0),
                        },
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    //background_color: BackgroundColor(colors.node_color),
                    ..default()
                },
                BlockUiMessageItem,
            ))
            .with_children(|innerca| {
                innerca.spawn(TextBundle::from_section(
                    message.username,
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color: colors.text_color,
                    },
                ));
            });
        builder
            .spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        margin: UiRect {
                            left: Val::Px(3.0),
                            right: Val::Px(3.0),
                            top: Val::Px(1.0),
                            bottom: Val::Px(10.0),
                        },
                        ..default()
                    },
                    background_color: BackgroundColor(colors.node_color),
                    ..default()
                },
                BlockUiMessageItem,
            ))
            .with_children(|innerc| {
                innerc.spawn(TextBundle::from_section(
                    message.message,
                    TextStyle {
                        font: font.clone(),
                        font_size,
                        color: colors.accent_color,
                    },
                ));
            });
    }
}

fn new_value_title(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    text: &str,
    font_size: f32,
    node_color: Color,
    text_color: Color,
    box_color: Color,
) {
    let mut row = builder.spawn(NodeBundle {
        style: Style {
            display: Display::Grid,
            grid_column: GridPlacement::span(2),
            padding: UiRect::all(Val::Px(1.0)),
            grid_template_columns: vec![GridTrack::auto(), GridTrack::auto(), GridTrack::auto()],
            grid_template_rows: vec![GridTrack::min_content()],

            ..default()
        },
        background_color: BackgroundColor(node_color),
        ..default()
    });

    row.with_children(|builder| {
        builder
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    justify_items: JustifyItems::End,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    grid_column: GridPlacement::span(2),
                    padding: UiRect::all(Val::Px(1.0)),
                    margin: UiRect {
                        left: Val::Px(60.0),
                        right: Val::Px(1.0),
                        top: Val::Px(1.0),
                        bottom: Val::Px(1.0),
                    },

                    ..default()
                },
                background_color: BackgroundColor(node_color),
                ..default()
            })
            .with_children(|innerc| {
                innerc.spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font,
                        font_size,
                        color: text_color,
                    },
                ));
            });
    });
    row.with_children(|builder| {
        builder
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    // justify_items: JustifyItems::End,
                    // align_items: AlignItems::Center,
                    // align_content: AlignContent::Center,
                    // justify_content: JustifyContent::Center,
                    grid_row: GridPlacement::span(1),
                    padding: UiRect::all(Val::Px(1.0)),

                    ..default()
                },
                background_color: BackgroundColor(node_color),
                ..default()
            })
            .with_children(|btn_color_shower| {
                btn_color_shower.spawn((
                    ButtonBundle {
                        style: Style {
                            // justify_content: JustifyContent::End,
                            // align_items: AlignItems::End,
                            // justify_items: JustifyItems::End,
                            // align_content: AlignContent::End,
                            margin: UiRect {
                                left: Val::Px(40.0),
                                right: Val::Px(1.0),
                                top: Val::Px(1.0),
                                bottom: Val::Px(1.0),
                            },
                            min_width: Val::Px(34.0),
                            max_width: Val::Px(34.0),
                            min_height: Val::Px(34.0),
                            max_height: Val::Px(34.0),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        border_color: BorderColor(Color::WHITE),
                        background_color: BackgroundColor(box_color), //node_color
                        ..default()
                    },
                    BtnShowingColor,
                ));
            });
    });
}

fn setup_ln_addr_menu_button(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    button_text: String,
    node_color: Color,
    button_color: Color,
    font_size: f32,
    edit_box_width: f32,
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
                min_width: Val::Px(edit_box_width),
                max_width: Val::Px(edit_box_width),
                min_height: Val::Px(36.0),
                max_height: Val::Px(36.0),
                //padding: UiRect::all(Val::Px(2.0)),
                margin: UiRect::bottom(Val::Px(12.0)),
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
                            min_width: Val::Px(edit_box_width - 2.0),
                            max_width: Val::Px(edit_box_width - 2.0),
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
                    EditabledTextBox,
                ))
                .with_children(|parent2| {
                    parent2.spawn((
                        TextBundle::from_section(
                            button_text.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size,
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
    font_size: f32,
    edit_box_width: f32,
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
                min_width: Val::Px(edit_box_width), //Val::Px(198.0),
                max_width: Val::Px(edit_box_width),
                min_height: Val::Px(36.0),
                max_height: Val::Px(36.0),
                //padding: UiRect::all(Val::Px(2.0)),
                margin: UiRect::bottom(Val::Px(12.0)),
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
                            min_width: Val::Px(edit_box_width - 2.0),
                            max_width: Val::Px(edit_box_width - 2.0),
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
                                font_size,
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
    font_size: f32,
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
                                font_size,
                                color: DEFAULT_NO_PICK_COLOR,
                            },
                        ),
                        NewBlockMessageText,
                    ));
                });
        });
}
