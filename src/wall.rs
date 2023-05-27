use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::{ball, collider, collision, out_wall, wall_location};

#[derive(Component)]
pub struct Wall;

pub fn spawn(
    commands: &mut Commands,
    location: wall_location::WallLocation,
    size: Vec2,
    thickness: f32,
    bundle: impl Bundle,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform {
                translation: location.position(size, thickness).extend(0.),
                scale: location.size(size, thickness).extend(0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall)
        .insert(collider::Collider)
        .insert(bundle);
}

pub fn collision_ball(
    ball_query: Query<&Transform, (With<ball::Ball>, With<collider::Collider>)>,
    wall_query: Query<
        (&Transform, Option<&out_wall::OutWall>),
        (With<Wall>, With<collider::Collider>),
    >,
    mut ball_reflection_event_writer: EventWriter<ball::ReflectionEvent>,
    mut ball_justify_event_writer: EventWriter<ball::JustifyEvent>,
) {
    let ball_transform = ball_query.single();

    wall_query.iter().for_each(|(transform, out_wall)| {
        let ball_collision = collide(
            transform.translation,
            transform.scale.truncate(),
            ball_transform.translation,
            ball_transform.scale.truncate(),
        );

        if let Some(ball_collision) = ball_collision {
            let ball_collision = collision::Collision::from(ball_collision);

            ball_reflection_event_writer.send(ball::ReflectionEvent { ball_collision });

            if out_wall.is_none() {
                ball_justify_event_writer.send(ball::JustifyEvent {
                    ball_collision,
                    transform: *transform,
                });
            }
        }
    });
}
