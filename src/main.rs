use std::{
    default,
    fmt::{self, format},
};

use bevy::{
    diagnostic::Diagnostics,
    prelude::*,
    tasks::{ComputeTaskPool, ParallelIterator},
    window::WindowResized,
};
use rand::Rng;

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup.system())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_system(framerate.system())
        // .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(snow_velocity.system())
        .add_system(update_position.system());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}

struct Snow;
struct Velocity(Vec2);

fn setup(
    commands: &mut Commands,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    let mut rng = rand::thread_rng();

    let window = windows.get_primary().unwrap();

    for _ in 0..200 {
        let x = rng.gen_range((-window.width() / 2.0)..(window.height() / 2.0));
        let y = rng.gen_range((-window.height() / 2.0)..(window.width() / 2.0));
        commands
            .spawn(SpriteBundle {
                material: Handle::default(),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                ..Default::default()
            })
            .with(Snow)
            .with(Velocity(randomish_velocity()));
    }
}

fn framerate(diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        info!("{:?}", fps.average())
    }
}

fn randomish_velocity() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-0.5..0.5);
    let y = rng.gen_range(-1.0..0.0);

    Vec2::new(x, y).normalize()
}

fn snow_velocity(pool: Res<ComputeTaskPool>, mut snow: Query<(&Snow, &mut Velocity)>) {
    // for (_snow, mut velocity) in

    snow.par_iter_mut(8)
        .for_each(&pool, |(_snow, mut velocity)| {
            let new_velocity = randomish_velocity();
            *velocity = Velocity(velocity.0 + new_velocity);
        });
}

fn update_position(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &mut Transform)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    for (mut velocity, mut transform) in q.iter_mut() {
        *transform = Transform::from_translation(
            transform.translation + time.delta_seconds() * velocity.0.extend(0.0),
        );

        if transform.translation.y < -window.height() / 2.0 {
            transform.translation.y += window.height();
            *velocity = Velocity(randomish_velocity());
        }

        if transform.translation.x > window.width() / 2.0 {
            transform.translation.x -= window.width();
            *velocity = Velocity(randomish_velocity());
        }

        if transform.translation.x < -window.width() / 2.0 {
            transform.translation.x += window.width();
            *velocity = Velocity(randomish_velocity());
        }
    }
}
