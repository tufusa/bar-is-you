use bevy::prelude::*;

pub struct Screen;
pub struct Block;
pub struct Ball;
pub struct Wall;

impl Screen {
    pub const SIZE: Vec2 = Vec2 { x: 860., y: 540. };
}

impl Block {
    pub const SIZE: Vec3 = Vec3 {
        x: 60.,
        y: 30.,
        z: 0.,
    };
    pub const GAP: f32 = 5.;
    pub const COLOR: Color = Color::rgb(1., 1., 1.);
}

impl Ball {
    const RADIUS: f32 = 10.;
    pub const SIZE: Vec3 = Vec3 {
        x: Ball::RADIUS * 2.,
        y: Ball::RADIUS * 2.,
        z: 0.,
    };
    pub const COLOR: Color = Color::rgb(1., 1., 1.);
}

impl Wall {
    pub const THICKNESS: f32 = 30.;
}
