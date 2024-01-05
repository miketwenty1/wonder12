use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetPersistencePolicy,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
};

use qrcode_generator::QrCodeEcc;

use crate::{
    componenty::{CancelQrButton, ClipboardBtn, ExpirationQrText},
    resourcey::{ColorPalette, InvoiceDataFromServer},
};

use super::QrInvoiceOverlay;

// asset_server: Res<AssetServer>,
// // mut explore_state: ResMut<NextState<ExploreState>>,
// // tile_cart: Res<TileCart>,
// // mut tile_cart_vec: ResMut<TileCartVec>,
// qr_data: Res<Qr>,

// #[derive(Component, Debug)]
// pub struct UiQrOverlay;

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

            let bevy_image = Image::new(
                extent,
                dimensions,
                data,
                img_format,
                RenderAssetPersistencePolicy::Keep,
            );

            let texture = images.add(bevy_image);
            Some(texture)
        }
        Err(e) => {
            info!("Failed to read the image: {:?}", e);
            None
        }
    };

    // main qr
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
            QrInvoiceOverlay,
        ))
        // the actual qr code box wrapper
        .with_children(|builder2| {
            let mut inner_builder = builder2.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(300.0),
                    height: Val::Px(360.0),
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    justify_items: JustifyItems::Center,
                    margin: UiRect::all(Val::Px(0.0)),
                    padding: UiRect::all(Val::Px(0.0)),
                    ..default()
                },
                background_color: BackgroundColor(colors.node_color),
                ..Default::default() // style: Style {
            });
            // top row
            inner_builder.with_children(|builder| {
                builder
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            width: Val::Percent(100.0),
                            // height: Val::Percent(100.0),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            margin: UiRect::right(Val::Px(10.0)),
                            padding: UiRect::all(Val::Px(0.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(colors.node_color),
                        ..default()
                    })
                    .with_children(|row_node| {
                        row_node
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    // height: Val::Percent(100.0),
                                    align_content: AlignContent::Center,
                                    justify_content: JustifyContent::End,
                                    align_items: AlignItems::Center,
                                    justify_items: JustifyItems::Center,
                                    margin: UiRect::right(Val::Px(0.0)),
                                    padding: UiRect::all(Val::Px(0.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(colors.node_color),
                                ..default()
                            })
                            .with_children(|left| {
                                left.spawn((
                                    TextBundle::from_section(
                                        "",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 16.0,
                                            color: colors.text_color,
                                        },
                                    ),
                                    ExpirationQrText,
                                ));
                            });
                    })
                    .with_children(|row_node| {
                        row_node
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(50.0),
                                    // height: Val::Percent(100.0),
                                    align_content: AlignContent::Center,
                                    justify_content: JustifyContent::End,
                                    align_items: AlignItems::Center,
                                    justify_items: JustifyItems::Center,
                                    margin: UiRect::right(Val::Px(0.0)),
                                    padding: UiRect::all(Val::Px(0.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(colors.node_color),
                                ..default()
                            })
                            .with_children(|right| {
                                right
                                    .spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(20.0),
                                                height: Val::Px(20.0),
                                                //justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                align_content: AlignContent::Center,
                                                justify_items: JustifyItems::Center,
                                                justify_content: JustifyContent::Center,
                                                ..default()
                                            },
                                            background_color: colors.button_color.into(),
                                            ..default()
                                        },
                                        CancelQrButton,
                                    ))
                                    .with_children(|ccbuilder| {
                                        ccbuilder.spawn(TextBundle::from_section(
                                            "X",
                                            TextStyle {
                                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                                font_size: 16.0,
                                                color: colors.text_color,
                                            },
                                        ));
                                    });
                            });
                    });
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
