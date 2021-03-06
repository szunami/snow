use std::{
    default,
    fmt::{self, format},
    time::Duration,
};

use bevy::{
    diagnostic::Diagnostics,
    prelude::*,
    tasks::{ComputeTaskPool, ParallelIterator},
    window::WindowResized,
};
use rand::{prelude::ThreadRng, Rng};
// use web_sys::console;

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(setup.system())
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_system(framerate.system())
        // .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(snow_velocity.system())
        .add_system(update_position.system())
        .add_system(animation.system());

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}

struct Snow;
struct Velocity(Vec2);

struct Skier;

struct SkiLoop(u32, Timer);

impl SkiLoop {
    fn increment(&mut self, delta_t: f32) {
        self.1.tick(delta_t);

        if self.1.finished() {
            self.0 = (self.0 + 1) % 12;
        }
    }
}

impl Default for SkiLoop {
    fn default() -> Self {
        SkiLoop(0, Timer::new(Duration::from_millis(100), true))
    }
}

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
            .with(Velocity(randomish_velocity(&mut rng)));
    }

    let skier_sprite = asset_server.load("textures/skiing.png");
    let skier_atlas = TextureAtlas::from_grid(skier_sprite, Vec2::new(80.0, 80.0), 12, 1);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(skier_atlas),
            transform: Transform::from_scale(Vec3::new(2.0, 2.0, 2.0)),
            ..Default::default()
        })
        .with(Skier)
        .with(SkiLoop::default());
}

fn framerate(diagnostics: Res<Diagnostics>) {
    if let Some(fps) = diagnostics.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS) {
        info!("{:?}", fps.average())
    }
}

fn randomish_velocity(rng: &mut ThreadRng) -> Vec2 {
    let x = rng.gen_range(-0.5..0.5);
    let y = rng.gen_range(-1.0..0.0);

    Vec2::new(x, y).normalize()
}

fn snow_velocity(mut snow: Query<(&Snow, &mut Velocity)>) {
    // let _timer = Timer::new("snow_velocity");

    let mut rng = rand::thread_rng();

    for (_snow, mut velocity) in snow.iter_mut() {
        let new_velocity = randomish_velocity(&mut rng);
        *velocity = Velocity(velocity.0 + new_velocity);
    }
}

fn update_position(
    time: Res<Time>,
    mut q: Query<(&mut Velocity, &mut Transform)>,
    windows: Res<Windows>,
) {
    // let _timer = Timer::new("update_position");

    let window = windows.get_primary().unwrap();

    let mut rng = rand::thread_rng();

    for (mut velocity, mut transform) in q.iter_mut() {
        *transform = Transform::from_translation(
            transform.translation + time.delta_seconds() * velocity.0.extend(0.0),
        );

        if transform.translation.y < -window.height() / 2.0 {
            transform.translation.y += window.height();
            *velocity = Velocity(randomish_velocity(&mut rng));
        }

        if transform.translation.x > window.width() / 2.0 {
            transform.translation.x -= window.width();
            *velocity = Velocity(randomish_velocity(&mut rng));
        }

        if transform.translation.x < -window.width() / 2.0 {
            transform.translation.x += window.width();
            *velocity = Velocity(randomish_velocity(&mut rng));
        }
    }
}

fn animation(t: Res<Time>, mut q: Query<(&mut TextureAtlasSprite, &mut SkiLoop)>) {
    for (mut sprite, mut ski_loop) in q.iter_mut() {
        ski_loop.increment(t.delta_seconds());
        sprite.index = ski_loop.0;
    }
}
