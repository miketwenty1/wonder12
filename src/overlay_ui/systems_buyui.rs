use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigButton, AllCartConfigText, BlockCostText, BlockHeightCartText, BuyMenuButton,
        CartButton, CurrentBlockDateText, CurrentBlockLnAddressText, CurrentBlockMessageText,
        CurrentBlockUsernameText, CurrentBlockValueText, EditabledTextBox, NewBlockColorButton,
        NewBlockColorText, NewBlockDataButton, NewBlockLnAddressButton, NewBlockLnAddressText,
        NewBlockMessageButton, NewBlockMessageText,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    eventy::BuyBlockRequest,
    keyboard::{resources::KeyboardData, KeyboardState},
    resourcey::{ColorPalette, CurrentCartBlock, KeyboardTarget, TargetType, TileCartVec, User},
    statey::ExploreState,
    utils::{convert_color_to_hexstring, is_valid_email_format_string},
    DisplayBuyUiState,
};

use all_colors::get_color_hex;

use super::layout_buy_menu::ButtonBack;

// const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// //const BUY_BUTTON: Color = Color::rgb(0.35, 0.50, 0.35);
// const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
// const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const FAIL_PRESSED_BUTTON: Color = Color::rgb(0.9, 0.1, 0.1);

//const INTRO_TEXT: &str = "This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!";
//const RANDOM_TEXT: &str = "This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!This game is in alpha, be prepared to lose all funds. Your lightning address must be correct to get a refund if someone steals your block!";

#[allow(clippy::type_complexity)]
pub fn leftright_cart_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CartButton),
        Changed<Interaction>,
    >,
    mut cart: ResMut<TileCartVec>,
    mut cart_item: ResMut<CurrentCartBlock>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<BlockHeightCartText>>,
        Query<&mut Text, With<CurrentBlockValueText>>,
        Query<&mut Text, With<CurrentBlockDateText>>,
        Query<&mut Text, With<CurrentBlockUsernameText>>,
        Query<&mut Text, With<CurrentBlockLnAddressText>>,
        Query<&mut Text, With<CurrentBlockMessageText>>,
        Query<&mut Text, With<BlockCostText>>,
    )>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color, button_comp) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let ind = cart.index;
                let color_string_hex = get_color_hex(&cart_item.color);
                //info!("index: {}, hex {}", cart.index, color_string_hex);
                let c = Color::hex(color_string_hex).unwrap();
                // info!(
                //     "lets see what the hex is after this {}",
                //     convert_color_to_hexstring(c)
                // );
                cart.vec[ind].new_color = c;
                cart.vec[ind].new_message = cart_item.message.to_string();

                *color = colors.light_color.into();

                match button_comp.0 {
                    -1 => {
                        let ind = if cart.index == 0 {
                            cart.vec.len() - 1
                        } else {
                            (cart.index - 1) % cart.vec.len()
                        };
                        cart.index = ind;
                    }
                    1 => {
                        let index = (cart.index + 1) % cart.vec.len();
                        cart.index = index;
                    }
                    _ => {
                        info!("button cart fail");
                    }
                };

                for mut text in param_set.p0().iter_mut() {
                    text.sections[0].value = format!("Block {}", cart.vec[cart.index].height);
                }
                for mut text in param_set.p1().iter_mut() {
                    text.sections[0].value = format!("Value: {} sats", cart.vec[cart.index].value);
                }
                for mut text in param_set.p2().iter_mut() {
                    let datetime_string = cart.vec[cart.index]
                        .event_date
                        .map_or("".to_string(), |datetime| {
                            datetime.date_naive().format("%Y-%m-%d").to_string()
                        });

                    text.sections[0].value = format!("Date: {}", datetime_string);
                }
                for mut text in param_set.p3().iter_mut() {
                    text.sections[0].value = format!("Owner: {}", cart.vec[cart.index].username);
                }
                for mut text in param_set.p4().iter_mut() {
                    text.sections[0].value = cart.vec[cart.index].ln_address.to_string();
                }
                for mut text in param_set.p5().iter_mut() {
                    text.sections[0].value = cart.vec[cart.index].message.to_string();
                }
                for mut text in param_set.p6().iter_mut() {
                    text.sections[0].value = format!("Cost: {} sats", cart.vec[cart.index].cost);
                }
                // for mut text in param_set.p7().iter_mut() {
                //     let r = convert_to_string(cart.vec[cart.index].new_color.r());
                //     let g = convert_to_string(cart.vec[cart.index].new_color.g());
                //     let b = convert_to_string(cart.vec[cart.index].new_color.b());
                //     let concat = format!("{}{}{}", r, g, b);
                //     let hex_color_string = all_colors::get_color_hex(concat.as_str());
                //     text.sections[0].value = hex_color_string;
                // }
                // for mut text in param_set..p8().iter_mut() {
                //     text.sections[0].value = cart.vec[cart.index].new_message.to_string();
                // }
                let a = all_colors::get_color_hex(&convert_color_to_hexstring(
                    cart.vec[cart.index].new_color,
                ));
                cart_item.color = a;
                cart_item.message = cart.vec[cart.index].new_message.to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn leftright_cart_button_system_set_new_text(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CartButton>),
    >,
    cart: Res<TileCartVec>,
    mut color_new_text: Query<&mut Text, (With<NewBlockColorText>, Without<NewBlockMessageText>)>,
    mut message_new_text: Query<&mut Text, (With<NewBlockMessageText>, Without<NewBlockColorText>)>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                for mut text in color_new_text.iter_mut() {
                    text.sections[0].value =
                        convert_color_to_hexstring(cart.vec[cart.index].new_color);
                }
                for mut text in message_new_text.iter_mut() {
                    text.sections[0].value = cart.vec[cart.index].new_message.to_string();
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn config_cart_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AllCartConfigButton>),
    >,
    mut cart: ResMut<TileCartVec>,
    mut config_text: Query<&mut Text, With<AllCartConfigText>>,
    cart_item: Res<CurrentCartBlock>,
    mut config: Local<bool>,
    // editable_textbox_q: Query<Entity, With<CouldBeEditabledTextBox>>,
    // mut commands: Commands,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                *config = !*config;
                for mut text in config_text.iter_mut() {
                    if *config {
                        text.sections[0].value = "X".to_string();
                        for block in cart.vec.iter_mut() {
                            block.new_color = Color::hex(get_color_hex(&cart_item.color)).unwrap();
                            block.new_message = cart_item.message.to_string();
                        }
                        // for button_textbox in editable_textbox_q.iter() {
                        //     commands.entity(button_textbox).remove::<EditabledTextBox>();
                        // }
                    } else {
                        text.sections[0].value = " ".to_string();
                        // for button_textbox in editable_textbox_q.iter() {
                        //     commands.entity(button_textbox).insert(EditabledTextBox);
                        // }
                    }
                }

                //     for mut text in color_new_text.iter_mut() {
                //         text.sections[0].value =
                //             convert_color_to_hexstring(cart.vec[cart.index].new_color);
                //     }
                //     for mut text in message_new_text.iter_mut() {
                //         text.sections[0].value = cart.vec[cart.index].new_message.to_string();
                //     }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn new_ln_address_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<NewBlockLnAddressButton>,
            //With<EditabledTextBox>,
        ),
    >,
    //mut text_query: Query<&mut Text, With<NewBlockLnAddressText>>,
    //keyboard_text: Res<KeyboardData>,
    mut target: ResMut<KeyboardTarget>,
    mut keyboard: ResMut<KeyboardData>,
    // text_query: Query<&Text, With<NewBlockLnAddressText>>,
    block_new_data: Res<CurrentCartBlock>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = colors.light_color.into();
                *target = KeyboardTarget(TargetType::NewLnAddress);

                keyboard.0 = block_new_data.ln_address.to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn new_color_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<NewBlockColorButton>,
            With<EditabledTextBox>,
        ),
    >,
    //mut text_query: Query<&mut Text, With<NewBlockLnAddressText>>,
    //keyboard_text: Res<KeyboardData>,
    mut target: ResMut<KeyboardTarget>,
    mut keyboard: ResMut<KeyboardData>,
    // text_query: Query<&Text, With<NewBlockColorText>>,
    block_new_data: Res<CurrentCartBlock>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = colors.light_color.into();
                *target = KeyboardTarget(TargetType::NewColor);
                keyboard.0 = block_new_data.color.to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn new_message_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<NewBlockMessageButton>,
            With<EditabledTextBox>,
        ),
    >,
    //mut text_query: Query<&mut Text, With<NewBlockLnAddressText>>,
    //keyboard_text: Res<KeyboardData>,
    mut target: ResMut<KeyboardTarget>,
    mut keyboard: ResMut<KeyboardData>,
    // text_query: Query<&Text, With<NewBlockMessageText>>,
    block_new_data: Res<CurrentCartBlock>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = colors.light_color.into();
                *target = KeyboardTarget(TargetType::NewMessage);
                keyboard.0 = block_new_data.message.to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn set_default_text_for_empty_text(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NewBlockDataButton>)>,
    //mut target: ResMut<KeyboardTarget>,
    //mut keyboard: ResMut<KeyboardData>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<NewBlockLnAddressText>>,
        Query<&mut Text, With<NewBlockColorText>>,
        Query<&mut Text, With<NewBlockMessageText>>,
    )>,
    block_new_data: Res<CurrentCartBlock>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if block_new_data.ln_address.is_empty() {
                    for mut text in param_set.p0().iter_mut() {
                        text.sections[0].value = DEFAULT_NEW_LN_TEXT.to_owned();
                        text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                    }
                }
                if block_new_data.color.is_empty() {
                    for mut text in param_set.p1().iter_mut() {
                        text.sections[0].value = DEFAULT_NEW_COLOR_TEXT.to_owned();
                        text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                    }
                }
                if block_new_data.message.is_empty() {
                    for mut text in param_set.p2().iter_mut() {
                        text.sections[0].value = DEFAULT_NEW_MESSAGE_TEXT.to_owned();
                        text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn buy_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyMenuButton>),
    >,

    mut buy_event: EventWriter<BuyBlockRequest>,
    // mut game_state: ResMut<NextState<DisplayBuyUiState>>,
    mut cart: ResMut<TileCartVec>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<NewBlockLnAddressText>>,
        Query<&mut Text, With<NewBlockColorText>>,
        Query<&mut Text, With<NewBlockMessageText>>,
    )>,
    mut user: ResMut<User>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        let index = cart.index;
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                for mut text in param_set.p0().iter_mut() {
                    let a = &text.sections[0].value;
                    if is_valid_email_format_string(a) {
                        user.ln_address = a.to_string();
                        *color = colors.light_color.into();
                        info!("yay!");
                        cart.vec[index].new_ln_address = text.sections[0].value.to_string();
                        buy_event.send(BuyBlockRequest);
                    } else {
                        *color = FAIL_PRESSED_BUTTON.into();
                        info!("poop!");
                        text.sections[0].style.color = FAIL_PRESSED_BUTTON;
                    }
                }

                for text in param_set.p1().iter() {
                    let c = all_colors::get_color_hex(&text.sections[0].value);
                    cart.vec[index].new_color = Color::hex(c).unwrap();
                }
                for text in param_set.p2().iter() {
                    cart.vec[index].new_message = text.sections[0].value.to_string();
                }

                //game_state.set(DisplayBuyUiState::On);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = Color::DARK_GREEN.into();
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
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                //text.sections[0].value = button_text;
                overlay_state.set(DisplayBuyUiState::Off);
                explore_state.set(ExploreState::On);
                keyboard_state.set(KeyboardState::Off);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}
