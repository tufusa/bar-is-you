use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::{ball, collider, collision, in_game, wall_location};

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
        (&Transform, Option<&in_game::InGame>),
        (With<Wall>, With<collider::Collider>),
    >,
    mut ball_reflection_event_writer: EventWriter<ball::ReflectionEvent>,
    mut ball_collision_wall_event_writer: EventWriter<ball::CollisionWallEvent>,
) {
    let ball_transform = ball_query.single();

    wall_query.iter().for_each(|(transform, in_game)| {
        let ball_collision = collide(
            transform.translation,
            transform.scale.truncate(),
            ball_transform.translation,
            ball_transform.scale.truncate(),
        );

        if let Some(ball_collision) = ball_collision {
            let ball_collision = collision::Collision::from(ball_collision);

            ball_reflection_event_writer.send(ball::ReflectionEvent { ball_collision });

            if in_game.is_some() {
                ball_collision_wall_event_writer.send(ball::CollisionWallEvent { ball_collision });
            }
        }
    });
}
