use bevy::{prelude::*, window::PresentMode};
use components::Velocity;
use player::PlayerPlugin;
use space::SpacePlugin;

mod components;
mod player;
mod space;

pub const RESOLUTION: f32 = 16. / 9.;
const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 400.;
const SPRITE_SCALE: f32 = 3.;

struct WindowSize {
    w: f32,
    h: f32,
}

fn main() {
    let height = 900.0;

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            title: "Space Flyer".to_string(),
            width: height * RESOLUTION,
            height,
            resizable: true,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(SpacePlugin)
        .add_startup_system(setup_system)
        .add_system(movement_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup_system(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();

    let (win_w, win_h) = (window.width(), window.height());

    commands.insert_resource(WindowSize { w: win_w, h: win_h });
}

fn movement_system(
    mut commands: Commands,
    window_size: Res<WindowSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform)>,
) {
    for (entity, velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        // despawn entity if outside of margin
        const MARGIN: f32 = 200.;

        if translation.y > window_size.h / 2. + MARGIN
            || translation.y < -window_size.h / 2. - MARGIN
            || translation.x > window_size.w / 2. + MARGIN
            || translation.x < -window_size.w / 2. - MARGIN
        {
            commands.entity(entity).despawn();
        }
    }
}
