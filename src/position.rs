use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0., y: 0. }
    }
}
