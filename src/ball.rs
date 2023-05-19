use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::{config, position};

#[derive(Component)]
struct Ball;

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pos: position::Position,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Circle::new(config::Ball::RADIUS).into())
            .into(),
        material: materials.add(ColorMaterial::from(config::Ball::COLOR)),
        
        ..default()
    });
}
