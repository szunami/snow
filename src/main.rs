use std::default;

use bevy::prelude::*;
use rand::Rng;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // .add_resource(AdjacencyGraph::default())
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(snow_velcity.system())
        .add_system(update_position.system())
        .add_system(clear_old_snow.system())
        .add_system(make_new_snow.system())
        .run();
}

struct Snow;
struct Velocity(Vec2);

fn setup(
    commands: &mut Commands,

    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());
}

fn randomish_velocity() -> Vec2 {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-0.5..0.5);
    let y = rng.gen_range(-1.0..0.0);

    Vec2::new(x, y).normalize()
}

fn snow_velcity(mut snow: Query<(&Snow, &mut Velocity)>) {
    for (_snow, mut velocity) in snow.iter_mut() {
        let new_velocity = randomish_velocity();

        *velocity = Velocity((velocity.0 + new_velocity));
    }
}

fn update_position(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in q.iter_mut() {
        *transform = Transform::from_translation(
            transform.translation + time.delta_seconds() * velocity.0.extend(0.0),
        )
    }
}

fn clear_old_snow(commands: &mut Commands, q: Query<(Entity, &Snow, &Transform)>) {
    for (snow_entity, _snow, transform) in q.iter() {
        if transform.translation.y < -200.0 {
            commands.despawn(snow_entity);
        }
    }
}

fn make_new_snow(commands: &mut Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x = rng.gen_range(-800.0..800.0);
        commands
            .spawn(SpriteBundle {
                material: Handle::default(),
                transform: Transform::from_translation(Vec3::new(x, 200.0, 0.0)),
                sprite: Sprite::new(Vec2::new(1.0, 1.0)),
                ..Default::default()
            })
            .with(Snow)
            .with(Velocity(randomish_velocity()));
    }
}
