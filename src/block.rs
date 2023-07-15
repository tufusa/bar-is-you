use bevy::{prelude::*, sprite::collide_aabb::*};
extern crate rand;
use rand::Rng;

use crate::{
    app_state::AppState, ball, collider, collision, config, position, rule, velocity, wall,
};

#[derive(Component)]
pub struct Block;

pub fn spawn(commands: &mut Commands, position: position::Position, bundle: impl Bundle) {
    let mut rng = rand::thread_rng();
    commands
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
        .insert(collider::Collider)
        .insert(bundle)
        .insert(velocity::Velocity {
            x: rng.gen_range(-200.0..=200.0),
            y: rng.gen_range(-200.0..=200.0),
        });
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

pub fn position_velocity(
    rule_server_query: Query<&rule::RuleServer>,
    time: Res<Time>,
    mut block_query: Query<(&mut position::Position, &velocity::Velocity), With<Block>>,
) {
    if rule_server_query.single().rule.is_move != rule::IsMove::Block {
        return;
    }

    let delta = time.delta_seconds();

    block_query.iter_mut().for_each(|(mut pos, velocity)| {
        pos.x += velocity.x * delta;
        pos.y += velocity.y * delta;
    });
}

pub fn collision_ball(
    mut commands: Commands,
    mut ball_query: Query<&Transform, (With<ball::Ball>, With<collider::Collider>)>,
    block_query: Query<(Entity, &Transform), (With<Block>, With<collider::Collider>)>,
    mut ball_reflection_event_writer: EventWriter<ball::ReflectionEvent>,
    rule_server_query: Query<&rule::RuleServer>,
    mut next_state: ResMut<NextState<AppState>>,
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
            if rule_server_query.single().rule.is_death == rule::IsDeath::Block {
                next_state.set(AppState::GameOver);
                return;
            }

            commands.entity(entity).despawn();
            ball_reflection_event_writer.send(ball::ReflectionEvent {
                ball_collision: collision::Collision::from(ball_collision),
            });
        }
    });
}

pub fn collision_wall(
    wall_query: Query<&Transform, (With<wall::Wall>, With<collider::Collider>)>,
    mut block_query: Query<
        (&Transform, &mut velocity::Velocity),
        (With<Block>, With<collider::Collider>),
    >,
    rule_server_query: Query<&rule::RuleServer>,
) {
    if rule_server_query.single().rule.is_move != rule::IsMove::Block {
        return;
    }

    wall_query.iter().for_each(|wall_transform| {
        block_query
            .iter_mut()
            .for_each(|(block_transform, mut velocity)| {
                let block_collision = collide(
                    wall_transform.translation,
                    wall_transform.scale.truncate(),
                    block_transform.translation,
                    block_transform.scale.truncate(),
                );

                if let Some(block_collision) = block_collision {
                    match block_collision {
                        Collision::Left => velocity.x *= if velocity.x < 0. { -1. } else { 1. },
                        Collision::Right => velocity.x *= if velocity.x > 0. { -1. } else { 1. },
                        Collision::Top => velocity.y *= if velocity.y > 0. { -1. } else { 1. },
                        Collision::Bottom => velocity.y *= if velocity.y < 0. { -1. } else { 1. },
                        Collision::Inside => {}
                    }
                }
            })
    });
}
