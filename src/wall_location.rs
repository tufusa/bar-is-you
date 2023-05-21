use crate::config;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    const WALL_X: f32 = config::Screen::SIZE.x / 2. + config::Wall::THICKNESS / 2.;
    const WALL_Y: f32 = config::Screen::SIZE.y / 2. + config::Wall::THICKNESS / 2.;

    pub fn position(self) -> Vec2 {
        match self {
            Self::Left => Vec2 {
                x: -Self::WALL_X,
                y: 0.,
            },
            Self::Right => Vec2 {
                x: Self::WALL_X,
                y: 0.,
            },
            Self::Top => Vec2 {
                x: 0.,
                y: Self::WALL_Y,
            },
            Self::Bottom => Vec2 {
                x: 0.,
                y: -Self::WALL_Y,
            },
        }
    }

    pub fn size(self) -> Vec2 {
        match self {
            Self::Left | Self::Right => Vec2 {
                x: config::Wall::THICKNESS,
                y: config::Screen::SIZE.y + config::Wall::THICKNESS * 2.,
            },
            Self::Top | Self::Bottom => Vec2 {
                x: config::Screen::SIZE.x + config::Wall::THICKNESS * 2.,
                y: config::Wall::THICKNESS,
            },
        }
    }
}
