use bevy::prelude::*;

pub struct Screen;
pub struct Block;
pub struct Ball;

impl Screen {
    pub const SIZE: Vec2 = Vec2 { x: 860., y: 540. };
}

impl Block {
    pub const SIZE: Vec2 = Vec2 { x: 60., y: 30. };
    pub const GAP: f32 = 5.;
    pub const COLOR: Color = Color::hex("FFFFFF").unwrap();
}

impl Ball {
    pub const RADIUS: f32 = 30.;
    pub const COLOR: Color = Color::hex("FFFFFF").unwrap();
}
