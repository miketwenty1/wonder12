use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
};

use qrcode_generator::QrCodeEcc;

use crate::{
    componenty::ClipboardBtn,
    resourcey::{ColorPalette, InvoiceDataFromServer},
};

use wasm_bindgen_futures::spawn_local;

// asset_server: Res<AssetServer>,
// // mut explore_state: ResMut<NextState<ExploreState>>,
// // tile_cart: Res<TileCart>,
// // mut tile_cart_vec: ResMut<TileCartVec>,
// qr_data: Res<Qr>,

#[derive(Component, Debug)]
pub struct UiQrOverlay;

pub fn spawn_qr(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    qr: Res<InvoiceDataFromServer>,
    asset_server: Res<AssetServer>,
    colors: Res<ColorPalette>,
) {
    let result: Vec<u8> =
        qrcode_generator::to_png_to_vec(&qr.invoice, QrCodeEcc::Low, 1024).unwrap();

    let img_result = image::load_from_memory(result.as_slice());

    let handle = match img_result {
        Ok(dynamic_image) => {
            let image_data = dynamic_image.to_rgba8();
            let (width, height) = image_data.dimensions();
            let data = image_data.into_raw();

            let extent = Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };

            let dimensions = TextureDimension::D2;

            let img_format = TextureFormat::bevy_default();

            let bevy_image = Image::new(extent, dimensions, data, img_format);

            let texture = images.add(bevy_image);
            Some(texture)
        }
        Err(e) => {
            info!("Failed to read the image: {:?}", e);
            None
        }
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::Rgba {
                    red: 1.,
                    green: 1.,
                    blue: 1.,
                    alpha: 0.,
                }),
                ..default()
            },
            UiQrOverlay,
        ))
        .with_children(|builder2| {
            let mut inner_builder = builder2.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(300.0),
                    height: Val::Px(350.0),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(colors.node_color),
                ..Default::default() // style: Style {
            });
            inner_builder.with_children(|builder| {
                builder.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(270.0),
                        height: Val::Px(270.0),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        margin: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    image: handle.unwrap().into(),
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default() // style: Style {
                });
            });
            inner_builder.with_children(|builder| {
                builder
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(80.0),
                                height: Val::Px(30.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            background_color: colors.button_color.into(),
                            ..default()
                        },
                        ClipboardBtn,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Copy Invoice",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: colors.text_color,
                            },
                        ));
                    });
            });
            inner_builder.with_children(|builder| {
                builder.spawn(TextBundle::from_section(
                    "Copy Button doesn't work on iOS",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 12.0,
                        color: colors.text_color,
                    },
                ));
            });
        });
}

#[allow(clippy::type_complexity, clippy::let_unit_value)]
pub fn clipboard_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ClipboardBtn>),
    >,
    // //mut text_query: Query<&mut Text>,
    // mut overlay_state: ResMut<NextState<DisplayBuyUiState>>,
    // mut explore_state: ResMut<NextState<ExploreState>>,
    //mut clip: ,
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
