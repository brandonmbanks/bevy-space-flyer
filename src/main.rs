use bevy::{prelude::*, window::PresentMode};

pub const RESOLUTION: f32 = 16.0 / 9.0;

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
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();

    let (win_w, win_h) = (window.width(), window.height());

    commands.insert_resource(WindowSize { w: win_w, h: win_h });
}
