use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::{collider, collision::Collision, config, position, rule, velocity};

#[derive(Component)]
pub struct Ball;
pub struct ReflectionEvent {
    pub ball_collision: Collision,
}

// めり込みを解除するイベント
pub struct JustifyEvent {
    pub ball_collision: Collision,
    pub transform: Transform,
}

pub fn spawn(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: position::Position,
    velocity: velocity::Velocity,
    bundle: impl Bundle,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            transform: Transform::from_scale(config::Ball::SIZE),
            material: materials.add(ColorMaterial::from(config::Ball::COLOR)),
            ..default()
        })
        .insert(Ball)
        .insert(position)
        .insert(velocity)
        .insert(collider::Collider)
        .insert(bundle);
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
        return;
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

pub fn justify_event_handler(
    mut ball_query: Query<&mut position::Position, (With<Ball>, With<collider::Collider>)>,
    mut ball_justify_event_reader: EventReader<JustifyEvent>,
) {
    let mut ball_position = ball_query.single_mut();

    ball_justify_event_reader.iter().for_each(|event| {
        justify_position(&event.ball_collision, &event.transform, &mut ball_position);
    })
}

fn justify_position(
    collision: &Collision,
    against_transform: &Transform,
    ball_position: &mut Mut<position::Position>,
) {
    let gap_x = against_transform.scale.x / 2. + config::Ball::SIZE.x / 2.;
    let gap_y = against_transform.scale.y / 2. + config::Ball::SIZE.y / 2.;

    let left_justified_x = against_transform.translation.x + gap_x;
    let right_justified_x = against_transform.translation.x - gap_x;
    let top_justified_y = against_transform.translation.y - gap_y;
    let bottom_justified_y = against_transform.translation.y + gap_y;

    match collision {
        Collision::Left => ball_position.x = ball_position.x.max(left_justified_x),
        Collision::Right => ball_position.x = ball_position.x.min(right_justified_x),
        Collision::Top => ball_position.y = ball_position.y.min(top_justified_y),
        Collision::Bottom => ball_position.y = ball_position.y.max(bottom_justified_y),
        Collision::Inside => {}
    }
}

pub fn input_position(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut ball_query: Query<(&mut position::Position, &velocity::Velocity), With<Ball>>,
    rule_server_query: Query<&rule::RuleServer>,
) {
    if rule_server_query.single().rule.is_you != rule::IsYou::Ball {
        return;
    }

    let (mut position, velocity) = ball_query.single_mut();
    let delta = time.delta_seconds();
    let (disp_x, disp_y) = (velocity.x.abs() * delta, velocity.y.abs() * delta);

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
