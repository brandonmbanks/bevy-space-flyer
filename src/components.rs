use bevy::{core::Timer, math::Vec2, prelude::Component};

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(val: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(val.0, val.1))
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Star;

#[derive(Component)]
pub struct Astroid;

pub struct AstroidTimer(pub Timer);

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Collider;
