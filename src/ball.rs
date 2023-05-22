use bevy::{
    prelude::*,
    sprite::{collide_aabb::*, ColorMaterial, MaterialMesh2dBundle},
};

use crate::{collider, config, position, velocity, rule};

#[derive(Component)]
pub struct Ball;
pub struct ReflectionEvent {
    pub ball_collision: Collision,
}

pub fn spawn(
    parent: &mut ChildBuilder,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: position::Position,
    velocity: velocity::Velocity,
) {
    parent
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            transform: Transform::from_scale(config::Ball::SIZE),
            material: materials.add(ColorMaterial::from(config::Ball::COLOR)),
            ..default()
        })
        .insert(Ball)
        .insert(position)
        .insert(velocity)
        .insert(collider::Collider);
}

pub fn transform_position(
    mut ball_query: Query<(&mut Transform, &position::Position), With<Ball>>,
) {
    let (mut transform, pos) = ball_query.single_mut();
    transform.translation = Vec3 {
        x: pos.x,
        y: pos.y,
        z: 0.,
    };
}

pub fn position_velocity(
    rule_server_query: Query<&rule::RuleServer>, 
    time: Res<Time>,
    mut ball_query: Query<(&mut position::Position, &velocity::Velocity), With<Ball>>,
) {
    if rule_server_query.single().rule.is_move != rule::IsMove::Ball {
        return
    }

    let (mut pos, velocity) = ball_query.single_mut();
    let delta = time.delta_seconds();

    pos.x += velocity.x * delta;
    pos.y += velocity.y * delta;
}

pub fn reflection_event_handler(
    mut ball_query: Query<&mut velocity::Velocity, (With<Ball>, With<collider::Collider>)>,
    mut ball_reflection_event_reader: EventReader<ReflectionEvent>,
) {
    let mut ball_velocity = ball_query.single_mut();

    ball_reflection_event_reader.iter().for_each(|event| {
        reflect(&event.ball_collision, &mut ball_velocity);
    });
}

fn reflect(collision: &Collision, velocity: &mut Mut<velocity::Velocity>) {
    let mut sign = Vec2 {
        x: velocity.x.signum(),
        y: velocity.y.signum(),
    };

    match collision {
        Collision::Left => sign.x = 1.,
        Collision::Right => sign.x = -1.,
        Collision::Top => sign.y = -1.,
        Collision::Bottom => sign.y = 1.,
        Collision::Inside => {}
    };

    velocity.x = sign.x * velocity.x.abs();
    velocity.y = sign.y * velocity.y.abs();
}
