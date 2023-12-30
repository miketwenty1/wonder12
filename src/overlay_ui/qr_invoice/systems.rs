use bevy::prelude::*;
use chrono::Utc;

use crate::{
    componenty::{CancelQrButton, ClipboardBtn, ExpirationQrText},
    resourcey::{ColorPalette, InvoiceDataFromServer},
    statey::{CommsApiState, DisplayBuyUiState, ExploreState},
};

use wasm_bindgen_futures::spawn_local;

#[allow(clippy::type_complexity, clippy::let_unit_value)]
pub fn clipboard_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClipboardBtn>),
    >,
    colors: Res<ColorPalette>,
    invoice_res: Res<InvoiceDataFromServer>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let invoice = invoice_res.invoice.to_string();
                *color = colors.light_color.into();
                let _task = spawn_local(async move {
                    let window = web_sys::window().expect("window"); // { obj: val };
                    let nav = window.navigator().clipboard();
                    match nav {
                        Some(a) => {
                            let p = a.write_text(&invoice);
                            let result = wasm_bindgen_futures::JsFuture::from(p).await;
                            match result {
                                Ok(_) => info!("clippyboy worked"),
                                Err(e) => info!("clipboard fail {:?}", e),
                            }
                        }
                        None => {
                            warn!("failed to copy clippyboy");
                        }
                    };
                });
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
pub fn cancel_qr_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CancelQrButton>),
    >,
    colors: Res<ColorPalette>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    mut ui_state: ResMut<NextState<DisplayBuyUiState>>,
    mut comms_state: ResMut<NextState<CommsApiState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                explore_state.set(ExploreState::On);
                ui_state.set(DisplayBuyUiState::Off);
                comms_state.set(CommsApiState::Off);
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

pub fn clean_up_qr(mut invoice_date: ResMut<InvoiceDataFromServer>) {
    invoice_date.code = "".to_string();
    invoice_date.expires = Utc::now();
    invoice_date.invoice = "".to_string();
}

pub fn expiration_text(
    mut text_query: Query<&mut Text, With<ExpirationQrText>>,
    invoice_res: Res<InvoiceDataFromServer>,
    colors: Res<ColorPalette>,
) {
    for mut text in text_query.iter_mut() {
        let time_left = (invoice_res.expires - Utc::now()).num_seconds();
        text.sections[0].value = format!("Expires in: {}", time_left);
        if time_left < 10 {
            text.sections[0].style.color = colors.red_color
        } else if time_left < 20 {
            text.sections[0].style.color = Color::YELLOW
        }
    }
}
