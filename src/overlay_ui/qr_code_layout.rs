use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
};

use qrcode_generator::QrCodeEcc;

use crate::resourcey::InvoiceDataFromServer;

// asset_server: Res<AssetServer>,
// // mut explore_state: ResMut<NextState<ExploreState>>,
// // tile_cart: Res<TileCart>,
// // mut tile_cart_vec: ResMut<TileCartVec>,
// qr_data: Res<Qr>,

pub fn spawn_qr(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    qr: Res<InvoiceDataFromServer>,
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
        .spawn((NodeBundle {
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
        },))
        .with_children(|builder2| {
            builder2
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(300.0),
                        height: Val::Px(300.0),
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        justify_items: JustifyItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::WHITE),
                    ..Default::default() // style: Style {
                })
                .with_children(|builder| {
                    builder.spawn(ImageBundle {
                        style: Style {
                            width: Val::Percent(90.0),
                            height: Val::Percent(90.0),
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            justify_items: JustifyItems::Center,
                            ..Default::default()
                        },
                        image: handle.unwrap().into(),
                        background_color: BackgroundColor(Color::WHITE),
                        ..Default::default() // style: Style {
                    });
                });
        });
}
