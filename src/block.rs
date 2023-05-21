use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::{ball, collider, config, position};

#[derive(Component)]
pub struct Block;

pub fn spawn(parent: &mut ChildBuilder, position: position::Position) {
    parent
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: config::Block::COLOR,
                ..Default::default()
            },
            transform: Transform::from_scale(config::Block::SIZE),
            ..Default::default()
        })
        .insert(Block)
        .insert(position)
        .insert(collider::Collider);
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

pub fn collision_ball(
    mut commands: Commands,
    mut ball_query: Query<&Transform, (With<ball::Ball>, With<collider::Collider>)>,
    block_query: Query<(Entity, &Transform), (With<Block>, With<collider::Collider>)>,
    mut ball_reflection_event_writer: EventWriter<ball::ReflectionEvent>,
) {
    let ball_transform = ball_query.single_mut();

    block_query.iter().for_each(|(entity, transform)| {
        let ball_collision = collide(
            transform.translation,
            transform.scale.truncate(),
            ball_transform.translation,
            ball_transform.scale.truncate(),
        );

        if let Some(ball_collision) = ball_collision {
            commands.entity(entity).despawn();
            ball_reflection_event_writer.send(ball::ReflectionEvent { ball_collision });
        }
    });
}
