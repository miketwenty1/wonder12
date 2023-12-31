use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigButton, AllCartConfigText, BlockCostText, BlockHeightCartText,
        BtnShowingColor, BuyMenuButton, CartButton, CurrentBlockDateText,
        CurrentBlockLnAddressText, CurrentBlockMessageText, CurrentBlockUsernameText,
        CurrentBlockValueText, EditabledTextBox, NewBlockColorButton, NewBlockColorText,
        NewBlockLnAddressButton, NewBlockLnAddressText, NewBlockMessageButton, NewBlockMessageText,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    eventy::BuyBlockRequest,
    keyboard::{resources::KeyboardData, KeyboardState},
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{ColorPalette, CurrentCartBlock, TargetType, TileCartVec, User},
    statey::ExploreState,
    utils::is_valid_email_format_string,
    DisplayBuyUiState,
};

use super::layout_buy_menu::ButtonBack;

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
    mut keyboard: ResMut<KeyboardData>,
) {
    for (interaction, mut color, button_comp) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if cart.vec.len() > 1 {
                    resolve_target_cart_data(&keyboard, &mut cart_item, &mut cart);
                    keyboard.target = TargetType::Nothing;

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

                    resolve_cart_item_data(&mut cart_item, &mut cart);

                    for mut text in param_set.p0().iter_mut() {
                        text.sections[0].value = format!("Block {}", cart.vec[cart.index].height);
                    }
                    for mut text in param_set.p1().iter_mut() {
                        text.sections[0].value =
                            format!("Value: {} sats", cart.vec[cart.index].value);
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
                        text.sections[0].value =
                            format!("Owner: {}", cart.vec[cart.index].username);
                    }
                    for mut text in param_set.p4().iter_mut() {
                        text.sections[0].value = cart.vec[cart.index].ln_address.to_string();
                    }
                    for mut text in param_set.p5().iter_mut() {
                        text.sections[0].value = cart.vec[cart.index].message.to_string();
                    }
                    for mut text in param_set.p6().iter_mut() {
                        text.sections[0].value =
                            format!("Cost: {} sats", cart.vec[cart.index].cost);
                    }

                    // let a = all_colors::get_color_hex(&convert_color_to_hexstring(
                    //     cart.vec[cart.index].new_color,
                    // ));
                    // if cart_item.color_text == DEFAULT_NEW_COLOR_TEXT {
                    //     cart_item.color = get_random_color();
                    // } else {
                    //     cart_item.color = cart.vec[cart.index].new_color
                    // }

                    cart_item.message = cart.vec[cart.index].new_message.to_string();
                    cart_item.color = cart.vec[cart.index].new_color;
                    cart_item.color_text = cart.vec[cart.index].new_color_text.to_string();
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                if cart.vec.len() > 1 {
                    *color = colors.accent_color.into();
                }
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.button_color.into();
            }
        }
    }
}

// the reason for this system fn is because the above query was getting complicated to prevent disjoint queries
#[allow(clippy::type_complexity)]
pub fn leftright_cart_button_system_set_new_text(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CartButton>)>,
    cart: Res<TileCartVec>,
    mut color_new_text: Query<&mut Text, (With<NewBlockColorText>, Without<NewBlockMessageText>)>,
    mut message_new_text: Query<&mut Text, (With<NewBlockMessageText>, Without<NewBlockColorText>)>,
    mut color_box: Query<&mut BackgroundColor, With<BtnShowingColor>>,
    //colors: Res<ColorPalette>,
) {
    for interaction in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if cart.vec.len() > 1 {
                    for mut text in color_new_text.iter_mut() {
                        if cart.vec[cart.index].new_color_text.to_string().is_empty() {
                            for mut color in color_box.iter_mut() {
                                *color = cart.vec[cart.index].new_color.into();
                                // text.sections[0].value = "".to_string(); // this shouldn't be needed
                                //                                          // TODO ^ see if we can remove this later
                            }
                        } else {
                            text.sections[0].value =
                                cart.vec[cart.index].new_color_text.to_string();
                        }

                        //convert_color_to_hexstring(cart.vec[cart.index].new_color);
                    }
                    for mut text in message_new_text.iter_mut() {
                        text.sections[0].value = cart.vec[cart.index].new_message.to_string();
                    }
                }
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                // *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                // *color = colors.button_color.into();
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
    mut cart_item: ResMut<CurrentCartBlock>,
    mut config: Local<bool>,
    // editable_textbox_q: Query<Entity, With<CouldBeEditabledTextBox>>,
    // mut commands: Commands,
    colors: Res<ColorPalette>,
    keyboard: Res<KeyboardData>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                resolve_target_cart_data(&keyboard, &mut cart_item, &mut cart);
                *config = !*config;
                for mut text in config_text.iter_mut() {
                    if *config {
                        text.sections[0].value = "X".to_string();
                        for block in cart.vec.iter_mut() {
                            block.new_color_text = cart_item.color_text.to_string();
                            block.new_color = cart_item.color; //Color::hex(get_color_hex(&cart_item.color)).unwrap();
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
    mut text_query: Query<&mut Text, With<NewBlockLnAddressText>>,
    //keyboard_text: Res<KeyboardData>,
    mut keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
    colors: Res<ColorPalette>,
    user: Res<User>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //text.sections[0].value = button_text;
                *color = colors.lite_button_color.into();
                resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
                keyboard.target = TargetType::NewLnAddress;
                keyboard.value = block_new_data.ln_address.to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                for mut text in text_query.iter_mut() {
                    if user.ln_address.len() > 4 {
                        text.sections[0].value = user.ln_address.to_string();
                        text.sections[0].style.color = colors.text_color;
                    }
                }
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
    mut keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.lite_button_color.into();
                resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
                keyboard.target = TargetType::NewColor;
                keyboard.value = block_new_data.color_text.to_string();
                //all_colors::get_color_hex(&block_new_data.color_text);
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
    mut keyboard: ResMut<KeyboardData>,
    //text_query: Query<&Text, With<NewBlockMessageText>>,
    colors: Res<ColorPalette>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.lite_button_color.into();
                resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
                keyboard.target = TargetType::NewMessage;
                keyboard.value = block_new_data.message.to_string();
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
    mut param_set: ParamSet<(
        Query<&mut Text, With<NewBlockLnAddressText>>,
        Query<&mut Text, With<NewBlockColorText>>,
        Query<&mut Text, With<NewBlockMessageText>>,
    )>,
    block_new_data: Res<CurrentCartBlock>,
    colors: Res<ColorPalette>,
) {
    if block_new_data.ln_address.is_empty() {
        for mut text in param_set.p0().iter_mut() {
            text.sections[0].value = DEFAULT_NEW_LN_TEXT.to_owned();
            text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
        }
    }
    if block_new_data.color_text.is_empty() {
        for mut text in param_set.p1().iter_mut() {
            text.sections[0].value = DEFAULT_NEW_COLOR_TEXT.to_owned();
            text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
        }
    } else {
        for mut text in param_set.p1().iter_mut() {
            text.sections[0].style.color = colors.text_color;
        }
    }
    if block_new_data.message.is_empty() {
        for mut text in param_set.p2().iter_mut() {
            text.sections[0].value = DEFAULT_NEW_MESSAGE_TEXT.to_owned();
            text.sections[0].style.color = DEFAULT_NO_PICK_COLOR;
        }
    } else {
        for mut text in param_set.p2().iter_mut() {
            text.sections[0].style.color = colors.text_color;
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn buy_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BuyMenuButton>),
    >,

    mut buy_event: EventWriter<BuyBlockRequest>,
    // mut game_state: ResMut<NextState<DisplayBuyUiState>>,
    mut cart: ResMut<TileCartVec>,
    mut cart_item: ResMut<CurrentCartBlock>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<NewBlockLnAddressText>>,
        Query<&mut Text, With<NewBlockColorText>>,
        Query<&mut Text, With<NewBlockMessageText>>,
    )>,
    mut user: ResMut<User>,
    colors: Res<ColorPalette>,
    mut toast: EventWriter<ToastEvent>,
    mut mouse: ResMut<Input<MouseButton>>,
    mut keyboard: ResMut<KeyboardData>,
) {
    for (interaction, mut color) in &mut interaction_query {
        let index = cart.index;
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                resolve_target_cart_data(&keyboard, &mut cart_item, &mut cart); //text.sections[0].value = button_text;
                for mut text in param_set.p0().iter_mut() {
                    let a = &text.sections[0].value;
                    if is_valid_email_format_string(a) {
                        user.ln_address = a.to_string();
                        *color = colors.light_color.into();
                        info!("yay!");
                        cart.vec[index].new_ln_address = text.sections[0].value.to_string();
                        buy_event.send(BuyBlockRequest);
                        // help with jumpiness when leaving this screen - hopefully
                        mouse.clear();
                        keyboard.target = TargetType::Nothing;
                        keyboard.value = "".to_string();
                    } else {
                        *color = colors.red_color.into();
                        info!("poop!");
                        text.sections[0].style.color = colors.red_color;
                        toast.send(ToastEvent {
                            ttype: ToastType::Bad,
                            message: "Please specify valid Lightning Address".to_string(),
                        })
                    }
                }

                // for text in param_set.p1().iter() {
                //     if text.sections[0].value == DEFAULT_NEW_COLOR_TEXT {
                //         cart.vec[index].new_color = get_random_color();
                //     } else {
                //         let c = all_colors::get_color_hex(&text.sections[0].value);
                //         cart.vec[index].new_color = Color::hex(c).unwrap();
                //     }
                // }
                // for text in param_set.p2().iter() {
                //     if text.sections[0].value == DEFAULT_NEW_MESSAGE_TEXT {
                //         cart.vec[index].new_message = "".to_string();
                //     } else {
                //         cart.vec[index].new_message = text.sections[0].value.to_string();
                //     }
                // }

                //game_state.set(DisplayBuyUiState::On);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.green_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
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
    mut mouse: ResMut<Input<MouseButton>>,
    mut keyboard: ResMut<KeyboardData>,
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
                // help with jumpiness when leaving this screen - hopefully
                mouse.clear();
                keyboard.target = TargetType::Nothing;
                keyboard.value = "".to_string();
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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn tab_key_system(
    keys: Res<Input<KeyCode>>,
    mut text_box_q: Query<
        (
            &mut Interaction,
            Option<&NewBlockLnAddressButton>,
            Option<&NewBlockColorButton>,
            Option<&NewBlockMessageButton>,
        ),
        With<EditabledTextBox>,
    >,
    keyboard: ResMut<KeyboardData>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        let tab_target = if keyboard.target == TargetType::NewLnAddress {
            TargetType::NewColor
        } else if keyboard.target == TargetType::NewColor {
            //keyboard.0 = block_new_data.color.to_string();
            TargetType::NewMessage
        } else if keyboard.target == TargetType::NewMessage
            || keyboard.target == TargetType::Nothing
        {
            //keyboard.0 = block_new_data.message.to_string();
            TargetType::NewLnAddress
        } else {
            TargetType::Nothing
        };

        for (mut interaction, ln_box, color_box, msg_box) in text_box_q.iter_mut() {
            match (ln_box, color_box, msg_box) {
                (Some(_), _, _) => {
                    if tab_target == TargetType::NewLnAddress {
                        *interaction = Interaction::Pressed;
                    } else {
                        *interaction = Interaction::None;
                    }
                }
                (_, Some(_), _) => {
                    if tab_target == TargetType::NewColor {
                        *interaction = Interaction::Pressed;
                    } else {
                        *interaction = Interaction::None;
                    }
                }
                (_, _, Some(_)) => {
                    if tab_target == TargetType::NewMessage {
                        *interaction = Interaction::Pressed;
                    } else {
                        *interaction = Interaction::None;
                    }
                }
                (None, None, None) => {
                    info!("Error05 this shouldn't of happened, please report this");
                }
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn show_color_button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<BtnShowingColor>>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    //text_query: Query<&mut Text, With<NewBlockColorText>>,
    // mut overlay_state: ResMut<NextState<DisplayBuyUiState>>,
    // mut explore_state: ResMut<NextState<ExploreState>>,
    // mut keyboard_state: ResMut<NextState<KeyboardState>>,
    // colors: Res<ColorPalette>,
    // mut mouse: ResMut<Input<MouseButton>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let text = &text_query.get_single().unwrap().sections[0].value;

        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {}
            Interaction::None => {
                if !block_new_data.color_text.is_empty() {
                    block_new_data.color =
                        Color::hex(all_colors::get_color_hex(&block_new_data.color_text)).unwrap();

                    *color = block_new_data.color.into();
                }
            }
        }
    }
}

pub fn resolve_target_cart_data(
    keyboard: &KeyboardData,
    block_new_data: &mut ResMut<CurrentCartBlock>,
    cart: &mut ResMut<TileCartVec>,
) {
    match keyboard.target {
        TargetType::Nothing => {
            // nothing to do
        }
        TargetType::NewLnAddress => {
            let index = cart.index;
            cart.vec[index].new_ln_address = keyboard.value.clone();
            block_new_data.ln_address = keyboard.value.to_string();
        }
        TargetType::NewColor => {
            let index = cart.index;
            cart.vec[index].new_color_text = keyboard.value.clone();
            cart.vec[index].new_color = block_new_data.color;
            block_new_data.color_text = keyboard.value.to_string();
        }
        TargetType::NewMessage => {
            let index = cart.index;
            cart.vec[index].new_message = keyboard.value.clone();
            block_new_data.message = keyboard.value.to_string();
        }
    }
}

pub fn resolve_cart_item_data(
    block_new_data: &mut ResMut<CurrentCartBlock>,
    cart: &mut ResMut<TileCartVec>,
) {
    let index = cart.index;
    //block_new_data.ln_address = cart.vec[index].new_ln_address.to_string();
    block_new_data.color_text = cart.vec[index].new_color_text.to_string();
    block_new_data.message = cart.vec[index].new_message.to_string();
}
