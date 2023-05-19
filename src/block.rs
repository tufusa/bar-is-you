use bevy::prelude::*;

use crate::{config, position};

#[derive(Component)]
pub struct Block;

pub fn spawn(commands: &mut Commands, position: position::Position) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                rect: Option::Some(Rect::new(
                    position.x,
                    position.y,
                    position.x + config::Block::SIZE.x,
                    position.y + config::Block::SIZE.y,
                )),
                color: config::Block::COLOR,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Block)
        .insert(position);
}

pub fn transform_position(
    mut block_query: Query<(&mut Transform, &position::Position), With<Block>>,
) {
    block_query
        .iter_mut()
        .for_each(|(mut transform, position)| {
            transform.translation = Vec3 {
                x: position.x,
                y: position.y,
                z: 0.,
            };
        });
}
