use bevy::prelude::*;

use crate::{ball, blocks, position, config, velocity, walls, bar};

pub fn setup(mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    blocks::spawn(&mut commands, 50., 15, 5);
    ball::spawn(
        &mut commands,
        meshes,
        materials,
        position::Position {
            x: 0.,
            y: -200. + config::Ball::SIZE.y,
        },
        velocity::Velocity { x: 220., y: 220. },
    );
    walls::spawn(&mut commands);
    bar::spawn(
        &mut commands,
        position::Position { x: 0., y: -200. },
        velocity::Velocity { x: 200., y: 200. },
    );
}