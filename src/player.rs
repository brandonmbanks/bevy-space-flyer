use bevy::prelude::*;

use crate::{
    components::{Player, Velocity},
    WindowSize, TIME_STEP,
};

const SPEED: f32 = 500.;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system);
        app.add_system(player_movement_system);
        app.add_system(player_input_system);
    }
}

fn spawn_player_system(mut commands: Commands, window_size: Res<WindowSize>) {
    let sprite = Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::splat(40.0)),
        ..default()
    };

    let left = -window_size.w / 2.0;

    commands
        .spawn_bundle(SpriteBundle {
            sprite,
            transform: Transform {
                translation: Vec3::new(left + 60.0, 0.0, 5.0),
                ..default()
            },
            ..default()
        })
        .insert(Player)
        .insert(Velocity { x: 0.0, y: 0.0 });
}

fn player_input_system(kb: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.y = if kb.pressed(KeyCode::Up) || kb.pressed(KeyCode::W) {
            1.
        } else if kb.pressed(KeyCode::Down) || kb.pressed(KeyCode::S) {
            -1.
        } else {
            0.
        };

        velocity.x = if kb.pressed(KeyCode::Right) || kb.pressed(KeyCode::D) {
            1.
        } else if kb.pressed(KeyCode::Left) || kb.pressed(KeyCode::A) {
            -1.
        } else {
            0.
        };
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        translation.x += velocity.x * TIME_STEP * SPEED;
        translation.y += velocity.y * TIME_STEP * SPEED;
    }
}
