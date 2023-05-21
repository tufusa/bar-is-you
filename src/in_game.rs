use bevy::prelude::*;

use crate::{
    app_state::AppState, ball, bar, block::Block, blocks, config, position, velocity, walls,
};

#[derive(Component)]
pub struct InGame;

pub fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(SpatialBundle::default())
        .insert(InGame)
        .with_children(|parent| spawn(parent, meshes, materials));
}

fn spawn(
    parent: &mut ChildBuilder,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    blocks::spawn(parent, 50., 15, 5);
    ball::spawn(
        parent,
        meshes,
        materials,
        position::Position {
            x: 0.,
            y: -200. + config::Ball::SIZE.y,
        },
        velocity::Velocity { x: 220., y: 220. },
    );
    walls::spawn(parent);
    bar::spawn(
        parent,
        position::Position { x: 0., y: -200. },
        velocity::Velocity { x: 200., y: 200. },
    );
}

pub fn check_clear(block_query: Query<&Block>, mut next_state: ResMut<NextState<AppState>>) {
    if block_query.is_empty() {
        next_state.set(AppState::GameClear);
    }
}

pub fn check_over(
    window_query: Query<&Window>,
    ball_query: Query<&position::Position, With<ball::Ball>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let max_y = window_query.single().height() / 2.;

    ball_query.iter().for_each(|pos| {
        if pos.y < -max_y {
            next_state.set(AppState::GameOver);
            return;
        }
    })
}

pub fn cleanup(mut commands: Commands, in_game_query: Query<Entity, With<InGame>>) {
    // in_game_query.iter().for_each(|in_game| {
    // commands.entity(in_game).despawn_recursive();
    // })
}
