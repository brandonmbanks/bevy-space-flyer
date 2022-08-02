use bevy::prelude::*;

use crate::{
    components::{Player, Velocity},
    WindowSize, BASE_SPEED, SPRITE_SCALE, TIME_STEP,
};

const PLAYER_SIZE: f32 = 48.;
const PLAYER_IMG: &str = "player.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, spawn_player_system);
        app.add_system(player_movement_system);
        app.add_system(player_input_system);
    }
}

fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_size: Res<WindowSize>,
) {
    let left = -window_size.w / 2.0;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(PLAYER_IMG),
            transform: Transform {
                translation: Vec3::new(left + 60.0, 0.0, 5.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
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

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
    window_size: Res<WindowSize>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;

        let new_x = translation.x + velocity.x * TIME_STEP * BASE_SPEED;
        let new_y = translation.y + velocity.y * TIME_STEP * BASE_SPEED;

        let left_bound = -window_size.w / 2. + PLAYER_SIZE / 2.;
        let right_bound = window_size.w / 2. - PLAYER_SIZE / 2.;
        let bottom_bound = -window_size.h / 2. + PLAYER_SIZE / 2.;
        let top_bound = window_size.h / 2. - PLAYER_SIZE / 2.;

        translation.x = new_x.clamp(left_bound, right_bound);
        translation.y = new_y.clamp(bottom_bound, top_bound);
    }
}
