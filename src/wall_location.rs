use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum WallLocation {
    Left,
    Right,
    Top,
    Bottom,
}

impl WallLocation {
    pub fn position(self, size: Vec2, thickness: f32) -> Vec2 {
        let wall_x = size.x / 2. + thickness / 2.;
        let wall_y = size.y / 2. + thickness / 2.;

        match self {
            Self::Left => Vec2 { x: -wall_x, y: 0. },
            Self::Right => Vec2 { x: wall_x, y: 0. },
            Self::Top => Vec2 { x: 0., y: wall_y },
            Self::Bottom => Vec2 { x: 0., y: -wall_y },
        }
    }

    pub fn size(self, size: Vec2, thickness: f32) -> Vec2 {
        match self {
            Self::Left | Self::Right => Vec2 {
                x: thickness,
                y: size.y + thickness * 2.,
            },
            Self::Top | Self::Bottom => Vec2 {
                x: size.x + thickness * 2.,
                y: thickness,
            },
        }
    }
}
