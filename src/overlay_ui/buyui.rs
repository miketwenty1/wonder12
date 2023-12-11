use bevy::prelude::*;

use crate::{
    componenty::{
        BlockHeightCartText, CurrentBlockDateText, CurrentBlockValueText, LeftCartButton,
        RightCartButton,
    },
    keyboard::{components::KeyboardNode, KeyboardState},
    resourcey::{TileCart, TileCartData, TileCartVec},
    statey::ExploreState,
    DisplayBuyUiState,
};

use super::UiOverlay;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

//const INTRO_TEXT: &str = "This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!";
const RANDOM_TEXT: &str = "This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!";

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
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Top-level grid (app frame)
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::FlexStart,
                    justify_items: JustifyItems::Stretch,
                    //align_items: AlignItems::Center,
                    grid_template_columns: vec![GridTrack::auto()],
                    grid_template_rows: vec![
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::min_content(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                        GridTrack::auto(),
                        //GridTrack::flex(1.0),
                    ],
                    // padding: UiRect {
                    //     left: Val::Percent(2.),
                    //     right: Val::Percent(2.),
                    //     top: Val::Percent(2.),
                    //     bottom: Val::Percent(2.),
                    // },
                    max_width: Val::Px(800.0),
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
            // block height left and right
            builder
                .spawn(NodeBundle {
                    style: Style {
                        //width: Val::Percent(100.0),
                        //height: Val::Px(25.0),
                        display: Display::Flex,
                        justify_items: JustifyItems::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..default()
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
            // current data
            builder
                .spawn(NodeBundle {
                    style: Style {
                        //width: Val::Percent(100.0),
                        //height: Val::Px(100.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(1.0)),
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
                        tile_cart_vec.vec[0].amount.to_string(),
                        datetime_string,
                        16.0,
                        Color::WHITE,
                    );
                    current_value_right(builder, font.clone(), RANDOM_TEXT, 16.0, Color::WHITE);
                    current_value_message(builder, font.clone(), RANDOM_TEXT, 16.0, Color::WHITE);
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
            //new block data
            builder
                .spawn(NodeBundle {
                    style: Style {
                        //width: Val::Percent(100.0),
                        //height: Val::Px(100.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(1.0)),
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
                        "Required:\ntype in a lightning address".to_string(),
                    );
                    setup_color_menu_button(
                        builder,
                        font.clone(),
                        "write in a color\nor get a random color".to_string(),
                    );
                    setup_message_menu_button(builder, font.clone(), "Write a message".to_string());
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
                        //width: Val::Percent(100.0),
                        //height: Val::Px(150.0),
                        display: Display::Grid,
                        justify_items: JustifyItems::Center,
                        padding: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::BLUE),
                    ..default()
                },
                KeyboardNode,
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

#[allow(clippy::type_complexity)]
pub fn buy_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonGo>),
    >,
    //mut text_query: Query<&mut Text>,
    //mut game_state: ResMut<NextState<DisplayBuyUiState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                //game_state.set(DisplayBuyUiState::On);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn back_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonBack>),
    >,
    //mut text_query: Query<&mut Text>,
    mut overlay_state: ResMut<NextState<DisplayBuyUiState>>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    mut keyboard_state: ResMut<NextState<KeyboardState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();
                overlay_state.set(DisplayBuyUiState::Off);
                explore_state.set(ExploreState::On);
                keyboard_state.set(KeyboardState::Off);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

// fn spawn_nested_text_bundle(
//     builder: &mut ChildBuilder,
//     font: Handle<Font>,
//     text: &str,
//     font_size: f32,
//     color: Color,
// ) {
//     builder.spawn(TextBundle::from_section(
//         text,
//         TextStyle {
//             font,
//             font_size,
//             color,
//         },
//     ));
// }

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
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
            LeftCartButton,
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
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
            RightCartButton,
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

// #[allow(clippy::type_complexity)]
// pub fn left_block_button_system(
//     mut interaction_query: Query<
//         (&Interaction, &mut BackgroundColor),
//         (Changed<Interaction>, With<ButtonBack>),
//     >,
//     //mut text_query: Query<&mut Text>,
//     mut overlay_state: ResMut<NextState<DisplayBuyUiState>>,
//     mut explore_state: ResMut<NextState<ExploreState>>,
//     mut keyboard_state: ResMut<NextState<KeyboardState>>,
// ) {
//     for (interaction, mut color) in &mut interaction_query {
//         //let mut text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 //text.sections[0].value = button_text;
//                 *color = PRESSED_BUTTON.into();
//                 overlay_state.set(DisplayBuyUiState::Off);
//                 explore_state.set(ExploreState::On);
//                 keyboard_state.set(KeyboardState::Off);
//             }
//             Interaction::Hovered => {
//                 //text.sections[0].value = button_text;
//                 *color = HOVERED_BUTTON.into();
//             }
//             Interaction::None => {
//                 //text.sections[0].value = button_text;
//                 *color = NORMAL_BUTTON.into();
//             }
//         }
//     }
// }

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
                // justify_items: JustifyItems::Start,
                // align_items: AlignItems::Start,
                // align_content: AlignContent::Start,
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

                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::GREEN),
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

fn setup_ln_addr_menu_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Start,
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

fn setup_color_menu_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Start,
                align_content: AlignContent::Start,

                padding: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::YELLOW),
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

fn setup_message_menu_button(builder: &mut ChildBuilder, font: Handle<Font>, button_text: String) {
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

#[allow(clippy::type_complexity)]
pub fn right_cart_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<RightCartButton>),
    >,
    mut cart: ResMut<TileCartVec>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<BlockHeightCartText>>,
        Query<&mut Text, With<CurrentBlockValueText>>,
        Query<&mut Text, With<CurrentBlockDateText>>,
    )>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // info!("pre index {}", cart.index);
                let index = (cart.index + 1) % cart.vec.len();
                cart.index = index;
                // info!("post index {}", cart.index);
                // info!("vec len {}", cart.vec.len());
                for mut text in param_set.p0().iter_mut() {
                    text.sections[0].value = format!("Block {}", cart.vec[cart.index].height);
                }
                for mut text in param_set.p1().iter_mut() {
                    text.sections[0].value = format!("Value {}", cart.vec[cart.index].amount);
                }
                for mut text in param_set.p2().iter_mut() {
                    let datetime_string = cart.vec[cart.index]
                        .event_date
                        .map_or("".to_string(), |datetime| {
                            datetime.date_naive().format("%Y-%m-%d").to_string()
                        });

                    text.sections[0].value = format!("Date {}", datetime_string);
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn left_cart_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<LeftCartButton>),
    >,
    mut cart: ResMut<TileCartVec>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<BlockHeightCartText>>,
        Query<&mut Text, With<CurrentBlockValueText>>,
        Query<&mut Text, With<CurrentBlockDateText>>,
    )>,
    // mut block_height:
    // mut current_value:
    // mut current_date:
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = PRESSED_BUTTON.into();

                let ind = if cart.index == 0 {
                    cart.vec.len() - 1
                } else {
                    (cart.index - 1) % cart.vec.len()
                };
                cart.index = ind;
                for mut text in param_set.p0().iter_mut() {
                    text.sections[0].value = format!("Block {}", cart.vec[cart.index].height);
                }
                for mut text in param_set.p1().iter_mut() {
                    text.sections[0].value = format!("Value {}", cart.vec[cart.index].amount);
                }
                for mut text in param_set.p2().iter_mut() {
                    let datetime_string = cart.vec[cart.index]
                        .event_date
                        .map_or("".to_string(), |datetime| {
                            datetime.date_naive().format("%Y-%m-%d").to_string()
                        });

                    text.sections[0].value = format!("Date {}", datetime_string);
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
