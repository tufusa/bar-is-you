use bevy::prelude::*;

use crate::{config, position};

#[derive(Component)]
pub struct Block;

pub fn spawn(mut commands: Commands, position: position::Position) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                rect: Option::Some(Rect::new(
                    position.x as f32,
                    position.y as f32,
                    position.x as f32 + config::Block::SIZE.x,
                    position.y as f32 + config::Block::SIZE.y,
                )),
                color: config::Block::COLOR,
                ..Default::default()
            },
            transform: Transform {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Block)
        .insert(position);
}

pub fn transform_position(mut block_query: Query<(&Block, &mut Transform, &position::Position)>) {
    block_query
        .iter_mut()
        .for_each(|(_block, mut transform, position)| {
            transform.translation = Vec3 {
                x: position.x as f32,
                y: position.y as f32,
                z: 0.,
            };
        });
}
