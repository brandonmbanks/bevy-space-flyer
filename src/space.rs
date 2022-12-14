use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    components::{Astroid, AstroidTimer, Star, Velocity},
    WindowSize, SPRITE_SCALE,
};

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_star_constellation_system);
        app.insert_resource(AstroidTimer(Timer::from_seconds(0.1, true)));
        app.add_system(spawn_stars_system);
        app.add_system(spawn_asteroids_system);
    }
}

fn spawn_stars_system(mut commands: Commands, window_size: Res<WindowSize>) {
    let right = window_size.w / 2.;

    let mut rng = thread_rng();

    let num_stars = rng.gen_range(0..3);

    for _ in 0..num_stars {
        let size = rng.gen_range(1.0..4.0);
        let rand_y = rng.gen_range((-window_size.h / 2.)..(window_size.h / 2.));

        let sprite = Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite,
                transform: Transform {
                    translation: Vec3::new(right + 10., rand_y, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Star)
            .insert(Velocity { x: -1.0, y: 0.0 });
    }
}

fn spawn_star_constellation_system(mut commands: Commands, window_size: Res<WindowSize>) {
    let mut rng = thread_rng();

    for _ in 0..250 {
        let rand_x = rng.gen_range((-window_size.w / 2.)..(window_size.w / 2.));
        let rand_y = rng.gen_range((-window_size.h / 2.)..(window_size.h / 2.));
        let size = rng.gen_range(1.0..4.0);

        let sprite = Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(size)),
            ..default()
        };

        commands
            .spawn_bundle(SpriteBundle {
                sprite,
                transform: Transform {
                    translation: Vec3::new(rand_x, rand_y, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Star)
            .insert(Velocity { x: -1.0, y: 0.0 });
    }
}

fn spawn_asteroids_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
    time: Res<Time>,
    mut timer: ResMut<AstroidTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();

        let sprites = [
            "asteroid_01.png",
            "asteroid_02.png",
            "asteroid_03.png",
        ];

        let img = sprites[rng.gen_range(0..sprites.len())];

        let right = window_size.w / 2.;

        for _ in 0..rng.gen_range(0..2) {
            let rand_y = rng.gen_range((-window_size.h / 2.)..(window_size.h / 2.));
            let scale = rng.gen_range(SPRITE_SCALE..7.0);

            commands
                .spawn_bundle(SpriteBundle {
                    texture: asset_server.load(img),
                    transform: Transform {
                        translation: Vec3::new(right + 100., rand_y, rng.gen_range(15.0..25.0)),
                        scale: Vec3::new(scale, scale, 1.),
                        ..default()
                    },
                    ..default()
                })
                .insert(Astroid)
                .insert(Velocity { x: -1.5, y: 0.0 });
        }
    }
}
