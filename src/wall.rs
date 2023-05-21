use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::{ball, collider, wall_location};

#[derive(Component)]
pub struct Wall;

pub fn spawn(commands: &mut Commands, location: wall_location::WallLocation) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: location.position().extend(0.),
                scale: location.size().extend(0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall)
        .insert(collider::Collider);
}

pub fn collision_ball(
    ball_query: Query<&Transform, (With<ball::Ball>, With<collider::Collider>)>,
    wall_query: Query<&Transform, (With<Wall>, With<collider::Collider>)>,
    mut ball_reflection_event_writer: EventWriter<ball::ReflectionEvent>,
) {
    let ball_transform = ball_query.single();

    wall_query.iter().for_each(|transform| {
        let ball_collision = collide(
            transform.translation,
            transform.scale.truncate(),
            ball_transform.translation,
            ball_transform.scale.truncate(),
        );

        if let Some(ball_collision) = ball_collision {
            ball_reflection_event_writer.send(ball::ReflectionEvent { ball_collision })
        }
    });
}
