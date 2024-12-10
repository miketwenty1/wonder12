use bevy::{prelude::*, text::FontSmoothing};

use crate::resourcey::ColorPalette;

use super::{ToastInnerNode, ToastNode, ToastText};

pub fn spawn_toast(
    commands: &mut Commands,
    colors: &Res<ColorPalette>,
    asset_server: &Res<AssetServer>,
    bg_color: Color,
    information_text: String,
) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center, // nope left right
                justify_items: JustifyItems::Center,
                margin: UiRect::top(Val::Percent(2.5)),
                ..default()
            },
            ToastNode,
            Visibility::Hidden,
        ))
        .with_children(|child| {
            // let bg_color2 = Color::Rgba {
            //     red: colors.accent_color.r(),
            //     green: colors.accent_color.g(),
            //     blue: colors.accent_color.b(),
            //     alpha: 0.8,
            // };
            child
                .spawn((
                    Node {
                        width: Val::Px(350.0),
                        height: Val::Px(65.0),
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center, //nope left right
                        justify_items: JustifyItems::Center,
                        //margin: UiRect::top(Val::Percent(30.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color.into()),
                    Visibility::Visible,
                    ToastInnerNode,
                ))
                .with_children(|childtext| {
                    childtext.spawn((
                        Text::new(information_text),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 22.0,
                            font_smoothing: FontSmoothing::AntiAliased,
                        },
                        TextColor(colors.text_color),
                        ToastText,
                    ));
                });
        });
}
