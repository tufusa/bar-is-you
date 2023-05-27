use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{ball, collider, collision, config, position, rule, velocity};

#[derive(Component)]
pub struct Bar;

pub fn spawn(
    commands: &mut Commands,
    position: position::Position,
    velocity: velocity::Velocity,
    bundle: impl Bundle,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: config::Bar::COLOR,
                ..Default::default()
            },
            transform: Transform::from_scale(config::Bar::SIZE),
            ..Default::default()
        })
        .insert(Bar)
        .insert(position)
        .insert(velocity)
        .insert(collider::Collider)
        .insert(bundle);
}

pub fn transform_position(mut bar_query: Query<(&mut Transform, &position::Position), With<Bar>>) {
    let (mut transform, position) = bar_query.single_mut();
    transform.translation = Vec3 {
        x: position.x,
        y: position.y,
        z: 0.,
    };
}

pub fn input_position(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut bar_query: Query<(&mut position::Position, &velocity::Velocity), With<Bar>>,
    rule_server_query: Query<&rule::RuleServer>,
) {
    if rule_server_query.single().rule.is_you != rule::IsYou::Bar {
        return;
    }

    let (mut position, velocity) = bar_query.single_mut();
    let delta = time.delta_seconds();
    let (disp_x, disp_y) = (velocity.x * delta, velocity.y * delta);

    let mut displacement = position::Position::new();

    if input.pressed(KeyCode::Right) {
        displacement.x += disp_x;
    }

    if input.pressed(KeyCode::Left) {
        displacement.x -= disp_x;
    }

    if input.pressed(KeyCode::Up) {
        displacement.y += disp_y;
    }

    if input.pressed(KeyCode::Down) {
        displacement.y -= disp_y;
    }

    position.x += displacement.x;
    position.y += displacement.y;
}

pub fn collision_ball(
    bar_query: Query<&Transform, (With<Bar>, With<collider::Collider>)>,
    ball_query: Query<&Transform, (With<ball::Ball>, With<collider::Collider>)>,
    mut ball_reflection_event_writer: EventWriter<ball::ReflectionEvent>,
) {
    let bar_transform = bar_query.single();
    let ball_transform = ball_query.single();

    let ball_collision = collide(
        bar_transform.translation,
        bar_transform.scale.truncate(),
        ball_transform.translation,
        ball_transform.scale.truncate(),
    );

    if let Some(ball_collision) = ball_collision {
        let ball_collision = collision::Collision::from(ball_collision);

        ball_reflection_event_writer.send(ball::ReflectionEvent { ball_collision });
    }
}
