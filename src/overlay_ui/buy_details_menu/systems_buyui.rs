use bevy::prelude::*;

use crate::{
    componenty::{
        AllCartConfigButton, AllCartConfigText, BlockCostText, BlockHeightCartText,
        BlockUiMessageItem, BtnShowingColor, BuyMenuButton, CartButton, CurrentBlockMessageNode,
        EditabledTextBox, HideMessageBtn, NewBlockColorButton, NewBlockColorText,
        NewBlockLnAddressButton, NewBlockLnAddressText, NewBlockMessageButton, NewBlockMessageText,
        ToggleButton, UiOverlayingExplorerButton,
    },
    consty::{
        DEFAULT_NEW_COLOR_TEXT, DEFAULT_NEW_LN_TEXT, DEFAULT_NEW_MESSAGE_TEXT,
        DEFAULT_NO_PICK_COLOR,
    },
    eventy::{BlockDetailMessage, BuyBlockRequest, MessageReceivedFromServer},
    keyboard::{resources::KeyboardData, KeyboardState},
    overlay_ui::toast::{ToastEvent, ToastType},
    resourcey::{
        ColorPalette, ConfigAllCartBlocks, CurrentCartBlock, TargetType, TileCartVec,
        ToggleVisible, User,
    },
    statey::ExploreState,
    utils::is_valid_email_format_string,
    DisplayBuyUiState,
};

use super::layout_buy_menu::{spawn_messages, ButtonBack};

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_messages_ui_system(
    cart: Res<TileCartVec>,
    colors: Res<ColorPalette>,
    message_item_q: Query<(Entity, &Children), With<BlockUiMessageItem>>,
    mut commands: Commands,
    message_placement_q: Query<Entity, With<CurrentBlockMessageNode>>,
    asset_server: Res<AssetServer>,
    mut messages_received: EventReader<MessageReceivedFromServer>,
) {
    for block_height in messages_received.read() {
        if block_height.0 == cart.vec[cart.index].height {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            for message_item in message_item_q.iter() {
                commands.entity(message_item.0).despawn_recursive();
            }

            match &cart.vec[cart.index].messages {
                Some(s) => {
                    for node in message_placement_q.iter() {
                        commands.entity(node).with_children(|child_builder| {
                            spawn_messages(
                                child_builder,
                                font.clone(),
                                s.to_vec(),
                                16.0,
                                colors.clone(),
                            );
                        });
                    }
                }
                None => {}
            };
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn leftright_cart_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CartButton),
        Changed<Interaction>,
    >,
    mut cart: ResMut<TileCartVec>,
    mut cart_item: ResMut<CurrentCartBlock>,
    mut param_set: ParamSet<(
        Query<&mut Text, With<BlockHeightCartText>>,
        // Query<&mut Text, With<CurrentBlockValueText>>,
        // Query<&mut Text, With<CurrentBlockDateText>>,
        // Query<&mut Text, With<CurrentBlockUsernameText>>,
        // Query<&mut Text, With<CurrentBlockLnAddressText>>,
        //Query<&mut Text, With<CurrentBlockMessageText>>,
        Query<&mut Text, With<BlockCostText>>,
    )>,
    colors: Res<ColorPalette>,
    mut keyboard: ResMut<KeyboardData>,
    mut ask_for_messages: EventWriter<BlockDetailMessage>,
    message_item_q: Query<(Entity, &Children), With<BlockUiMessageItem>>,
    mut commands: Commands,
    message_placement_q: Query<Entity, With<CurrentBlockMessageNode>>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut color, button_comp) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                if cart.vec.len() > 1 {
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
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
                    // for mut text in param_set.p1().iter_mut() {
                    //     text.sections[0].value =
                    //         format!("Value: {} sats", cart.vec[cart.index].value);
                    // }
                    // for mut text in param_set.p2().iter_mut() {
                    //     let datetime_string = cart.vec[cart.index]
                    //         .event_date
                    //         .map_or("".to_string(), |datetime| {
                    //             datetime.date_naive().format("%Y-%m-%d").to_string()
                    //         });

                    //     text.sections[0].value = format!("Date: {}", datetime_string);
                    // }
                    // for mut text in param_set.p3().iter_mut() {
                    //     text.sections[0].value =
                    //         format!("Owner: {}", cart.vec[cart.index].username);
                    // }
                    // for mut text in param_set.p4().iter_mut() {
                    //     text.sections[0].value = cart.vec[cart.index].ln_address.to_string();
                    // }
                    for message_item in message_item_q.iter() {
                        commands.entity(message_item.0).despawn_recursive();
                    }

                    match &cart.vec[cart.index].messages {
                        Some(s) => {
                            for node in message_placement_q.iter() {
                                commands.entity(node).with_children(|child_builder| {
                                    spawn_messages(
                                        child_builder,
                                        font.clone(),
                                        s.to_vec(),
                                        16.0,
                                        colors.clone(),
                                    );
                                });
                            }
                        }
                        None => {}
                    };

                    match cart.vec[cart.index].value {
                        0 => {
                            // 0: show nothing, 128: show current owner
                            // Since both cases are effectively no-ops in your example, they're combined here.
                            // You may need to adjust this based on actual logic you want to implement for value 128.
                            //info!("no messages to show");
                        }
                        128 => {
                            //info!("current message to show"); eventually if it makes sense we can load the first message first
                        }
                        _ => {
                            if let Some(messages) = &cart.vec[cart.index].messages {
                                if messages.len() == 1 {
                                    info!("querying for messages to show");
                                    ask_for_messages
                                        .send(BlockDetailMessage(cart.vec[cart.index].height));
                                }
                            }
                        }
                    }

                    for mut text in param_set.p1().iter_mut() {
                        text.sections[0].value =
                            format!("Cost: {} sats", cart.vec[cart.index].cost);
                    }

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
                    }
                    for mut text in message_new_text.iter_mut() {
                        text.sections[0].value = cart.vec[cart.index].new_message.to_string();
                    }
                }
            }
            Interaction::Hovered => {}
            Interaction::None => {}
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
    mut cart_config: ResMut<ConfigAllCartBlocks>,
    mut cart_item: ResMut<CurrentCartBlock>,
    colors: Res<ColorPalette>,
    keyboard: Res<KeyboardData>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                resolve_target_cart_data(&keyboard, &mut cart_item, &mut cart);
                cart_config.0 = !cart_config.0;
                for mut text in config_text.iter_mut() {
                    if cart_config.0 {
                        text.sections[0].value = "X".to_string();
                        for block in cart.vec.iter_mut() {
                            block.new_color_text = cart_item.color_text.to_string();
                            block.new_color = cart_item.color;
                            block.new_message = cart_item.message.to_string();
                        }
                    } else {
                        text.sections[0].value = " ".to_string();
                    }
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn hide_message_btn_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HideMessageBtn>),
    >,
    mut message_items: Query<&mut Style, With<CurrentBlockMessageNode>>,
    colors: Res<ColorPalette>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                for mut style in message_items.iter_mut() {
                    if style.display != Display::None {
                        style.display = Display::None;
                    } else {
                        style.display = Display::Grid;
                    }
                }
            }
            Interaction::Hovered => {
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                *color = colors.button_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn new_ln_address_button_system(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<NewBlockLnAddressButton>),
    >,

    mut keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
                keyboard.target = TargetType::NewLnAddress;
                keyboard.value = block_new_data.ln_address.to_string();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn new_color_button_system(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<NewBlockColorButton>,
            With<EditabledTextBox>,
        ),
    >,

    mut keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
                keyboard.target = TargetType::NewColor;
                keyboard.value = block_new_data.color_text.to_string();
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn new_message_button_system(
    mut interaction_query: Query<
        &Interaction,
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
    //colors: Res<ColorPalette>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                //*color = colors.lite_button_color.into();
                resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
                keyboard.target = TargetType::NewMessage;
                keyboard.value = block_new_data.message.to_string();
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                //*color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                // if keyboard.target == TargetType::NewMessage {
                //     *color = colors.lite_button_color.into();
                // } else {
                //     *color = colors.button_color.into();
                // }
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
    mut mouse: ResMut<ButtonInput<MouseButton>>,
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
                        });
                    }
                }
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
    mut mouse: ResMut<ButtonInput<MouseButton>>,
    mut keyboard: ResMut<KeyboardData>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                //text.sections[0].value = button_text;
                explore_state.set(ExploreState::On);
                keyboard_state.set(KeyboardState::Off);
                // help with jumpiness when leaving this screen - hopefully
                mouse.clear();
                keyboard.target = TargetType::Nothing;
                keyboard.value = "".to_string();
                overlay_state.set(DisplayBuyUiState::Off);
            }
            Interaction::Hovered => {
                //text.sections[0].value = button_text;
                *color = colors.accent_color.into();
            }
            Interaction::None => {
                //text.sections[0].value = button_text;
                *color = colors.red_color.into();
            }
        }
    }
}

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn tab_key_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut keyboard: ResMut<KeyboardData>,
    mut block_new_data: ResMut<CurrentCartBlock>,
    mut tile_cart: ResMut<TileCartVec>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        resolve_target_cart_data(&keyboard, &mut block_new_data, &mut tile_cart);
        match keyboard.target {
            TargetType::NewLnAddress => {
                keyboard.value = block_new_data.color_text.to_string();
                keyboard.target = TargetType::NewColor;
            }
            TargetType::Nothing => {
                keyboard.value = block_new_data.ln_address.to_string();
                keyboard.target = TargetType::NewLnAddress;
            }
            TargetType::NewColor => {
                keyboard.value = block_new_data.message.to_string();
                keyboard.target = TargetType::NewMessage;
            }
            TargetType::NewMessage => {
                keyboard.value = block_new_data.ln_address.to_string();
                keyboard.target = TargetType::NewLnAddress;
            }
            _ => {}
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn highlight_box_system(
    mut text_box_q: Query<
        (
            &mut BackgroundColor,
            Option<&NewBlockLnAddressButton>,
            Option<&NewBlockColorButton>,
            Option<&NewBlockMessageButton>,
        ),
        With<EditabledTextBox>,
    >,
    keyboard: ResMut<KeyboardData>,
    colors: Res<ColorPalette>,
) {
    for (mut color, ln_box, color_box, msg_box) in text_box_q.iter_mut() {
        match (ln_box, color_box, msg_box) {
            (Some(_), _, _) => {
                if keyboard.target == TargetType::NewLnAddress {
                    *color = colors.lite_button_color.into();
                } else {
                    *color = colors.button_color.into();
                }
            }
            (_, Some(_), _) => {
                if keyboard.target == TargetType::NewColor {
                    *color = colors.lite_button_color.into();
                } else {
                    *color = colors.button_color.into();
                }
            }
            (_, _, Some(_)) => {
                if keyboard.target == TargetType::NewMessage {
                    *color = colors.lite_button_color.into();
                } else {
                    *color = colors.button_color.into();
                }
            }
            (None, None, None) => {
                info!("Error05 this shouldn't of happened, please report this");
            }
        }
    }
}
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn show_color_button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), With<BtnShowingColor>>,
    mut block_new_data: ResMut<CurrentCartBlock>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let text = &text_query.get_single().unwrap().sections[0].value;

        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {}
            Interaction::None => {
                if !block_new_data.color_text.is_empty() {
                    block_new_data.color =
                        LegacyColor::hex(all_colors::get_color_hex(&block_new_data.color_text))
                            .unwrap();

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
        _ => {}
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

pub fn show_ui_buttons(
    mut ui_buttons: Query<&mut Visibility, With<ToggleButton>>,
    mut toggle_ui_buttons: Query<
        &mut Visibility,
        (With<UiOverlayingExplorerButton>, Without<ToggleButton>),
    >,
    toggle_visible: Res<ToggleVisible>,
) {
    for mut button in toggle_ui_buttons.iter_mut() {
        *button = Visibility::Visible;
    }

    if toggle_visible.0 {
        for mut button in ui_buttons.iter_mut() {
            *button = Visibility::Visible;
        }
    }
}
