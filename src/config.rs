use bevy::prelude::*;

use crate::rule;

pub struct Title;
pub struct Field;
pub struct Block;
pub struct Ball;
pub struct Wall;
pub struct Bar;
pub struct Rule;

impl Title {
    pub const TITLE: &str = "Bar Is You";
}

impl Field {
    pub const SIZE: Vec2 = Vec2 { x: 1200., y: 700. };
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
    pub const THICKNESS: f32 = 15.;
}

impl Bar {
    pub const SIZE: Vec3 = Vec3 {
        x: 250.,
        y: 10.,
        z: 0.,
    };
    pub const COLOR: Color = Color::rgb(1., 1., 1.);
}

impl Rule {
    pub const INIT: rule::Rule = rule::Rule {
        is_you: rule::IsYou::Ball,
        is_death: rule::IsDeath::Block,
        is_win: rule::IsWin::BreakAll,
        is_move: rule::IsMove::Block,
    };
}
