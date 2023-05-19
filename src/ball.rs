use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::{config, position};

#[derive(Component)]
pub struct Ball;

pub fn spawn(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    position: position::Position,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(config::Ball::RADIUS).into())
                .into(),
            material: materials.add(ColorMaterial::from(config::Ball::COLOR)),
            ..default()
        })
        .insert(Ball)
        .insert(position);
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
