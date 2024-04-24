use bevy::prelude::*;
use chrono::Utc;
use web_sys::{Blob, BlobPropertyBag};

use crate::{
    componenty::{CancelQrButton, ClipboardBtn, ExpirationQrText},
    eventy::{HideBackupCopyBtn, ShowBackupCopyBtn},
    resourcey::{ColorPalette, InvoiceDataFromServer, IsIphone},
    statey::{CommsApiState, DisplayBuyUiState, ExploreSelectState, ExploreState},
};

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    js_sys::{self, Array, Object},
    spawn_local, wasm_bindgen,
};

#[wasm_bindgen]
extern "C" {
    type NavigatorExt;

    #[wasm_bindgen(method, getter, js_name = userActivation)]
    fn user_activation(this: &NavigatorExt) -> UserActivation;

    type UserActivation;

    #[wasm_bindgen(method, getter, js_name = isActive)]
    fn is_active(this: &UserActivation) -> bool;

    type ClipboardItem;

    #[wasm_bindgen(constructor)]
    fn new(data: &Object) -> ClipboardItem;

    type HTMLParagraphElement;

    #[wasm_bindgen(method, setter = innerText)]
    fn set_inner_text(this: &HTMLParagraphElement, text: &str);

    #[wasm_bindgen(js_namespace = document)]
    fn query_selector(selector: &str) -> Option<web_sys::Element>;

}

#[allow(clippy::type_complexity, clippy::let_unit_value)]
pub fn clipboard_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClipboardBtn>),
    >,
    colors: Res<ColorPalette>,
    invoice_res: Res<InvoiceDataFromServer>,
    mut event: EventWriter<ShowBackupCopyBtn>,
    iphone: Res<IsIphone>,
) {
    for (interaction, mut color) in &mut interaction_query {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                let invoice = invoice_res.invoice.to_string();
                *color = colors.light_color.into();

                if iphone.0 {
                    event.send(ShowBackupCopyBtn);
                } else {
                    let _task = spawn_local(async move {
                        let window: web_sys::Window = web_sys::window().expect("window"); // { obj: val };

                        let nav = window.navigator().clipboard();
                        match nav {
                            Some(a) => {
                                let navigator: NavigatorExt =
                                    web_sys::window().unwrap().navigator().unchecked_into();
                                let is_active: bool = navigator.user_activation().is_active();
                                info!("is_active? {}", is_active);
                                let is_secure = window.is_secure_context();
                                info!("is_secure_context? {}", is_secure);

                                let p = a.write_text(&invoice);

                                let result = wasm_bindgen_futures::JsFuture::from(p).await;

                                match result {
                                    Ok(_) => {
                                        info!("clippyboy worked")
                                    }
                                    Err(e) => {
                                        info!("clipboard fail {:?}", e);
                                        let item_data = Object::new();
                                        let item_value = Blob::new_with_blob_sequence_and_options(
                                            &Array::of1(&"clipboard test".into()),
                                            BlobPropertyBag::new().type_("text/plain"),
                                        )
                                        .unwrap();
                                        js_sys::Reflect::set(
                                            &item_data,
                                            &"text/plain".into(),
                                            &item_value,
                                        )
                                        .unwrap();
                                        let item = ClipboardItem::new(&item_data);
                                        let p2 = a.write(&Array::of1(&item));
                                        let result2 =
                                            wasm_bindgen_futures::JsFuture::from(p2).await;

                                        match result2 {
                                            Ok(_) => {
                                                info!("second copy method worked");
                                            }
                                            Err(e) => {
                                                info!("second copy method also failed {:#?}, going to give you a html copy button instead", e);
                                            }
                                        }
                                    }
                                }
                            }
                            None => {
                                warn!("failed to get a clipboard");
                            }
                        };
                    });
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

#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn cancel_qr_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CancelQrButton>),
    >,
    colors: Res<ColorPalette>,
    mut explore_state: ResMut<NextState<ExploreState>>,
    mut explore_select_state: ResMut<NextState<ExploreSelectState>>,
    mut ui_state: ResMut<NextState<DisplayBuyUiState>>,
    mut comms_state: ResMut<NextState<CommsApiState>>,
    mut event: EventWriter<HideBackupCopyBtn>,
    iphone: Res<IsIphone>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = colors.light_color.into();
                explore_state.set(ExploreState::On);
                explore_select_state.set(ExploreSelectState::On);
                ui_state.set(DisplayBuyUiState::Off);
                comms_state.set(CommsApiState::Off);
                if iphone.0 {
                    event.send(HideBackupCopyBtn);
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
