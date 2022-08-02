use bevy::{core::Timer, prelude::Component};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Astroid;

pub struct AstroidTimer(pub Timer);
