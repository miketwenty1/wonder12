use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 4.0,
            far: 1000.,
            near: -1000.,
            ..Default::default()
        },
        ..Default::default()
    });

    // for y in 1..500 {
    //     for x in 1..500 {
    //         let mut rng = rand::thread_rng();
    //         let r: f32 = rng.gen_range(0.0..=1.0);
    //         let g: f32 = rng.gen_range(0.0..=1.0);
    //         let b: f32 = rng.gen_range(0.0..=1.0);

    //         commands.spawn(SpriteBundle {
    //             sprite: Sprite {
    //                 color: Color::rgb(r, g, b),
    //                 custom_size: Some(Vec2::new(10.0, 10.0)),
    //                 ..default()
    //             },
    //             transform: Transform::from_translation(Vec3::new(
    //                 (-10. * 250.0) + (x as f32 * 10.),
    //                 (-10. * 250.0) + (y as f32 * 10.),
    //                 0.,
    //             )),
    //             ..default()
    //         });
    //     }
    // }
}
