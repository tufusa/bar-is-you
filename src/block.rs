use bevy::prelude::*;

use crate::{config, position};

#[derive(Component)]
struct Block;

pub fn spawn(commands: &mut Commands, position: position::Position) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                rect: Option::Some(Rect::new(
                    position.x as f32,
                    position.y as f32,
                    (position.x + config::BLOCK_WIDTH) as f32,
                    (position.y + config::BLOCK_HEIGHT) as f32,
                )),
                ..Default::default()
            },
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Block)
        .insert(position::Position { x: 0, y: 0 });
}
