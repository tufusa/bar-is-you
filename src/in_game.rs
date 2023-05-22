use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    app_state::AppState, ball, bar, block::Block, blocks, config, field, font, out_wall, position,
    rule, ui, velocity,
};

#[derive(Component, Clone, Copy)]
pub struct InGame;

pub fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    ui_base_query: Query<Entity, With<ui::Base>>,
    rule_font: Res<font::Rule>,
    window_query: Query<&Window>,
) {
    commands
        .spawn(SpatialBundle::default())
        .insert(InGame)
        .with_children(|parent| spawn(parent, meshes, materials, window_query.single()));

    rule::spawn_server(&mut commands, config::Rule::INIT, InGame);

    commands
        .entity(ui_base_query.single())
        .with_children(|parent| rule::spawn_displayer(parent, rule_font, InGame));
}

fn spawn(
    parent: &mut ChildBuilder,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    window: &Window,
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
    field::spawn(parent, InGame);
    bar::spawn(
        parent,
        position::Position { x: 0., y: -200. },
        velocity::Velocity { x: 200., y: 200. },
    );
    out_wall::spawn(parent, window, InGame);
}

pub fn check_clear(block_query: Query<&Block>, mut next_state: ResMut<NextState<AppState>>) {
    if block_query.is_empty() {
        next_state.set(AppState::GameClear);
    }
}

pub fn check_over(
    out_wall_query: Query<&Transform, With<out_wall::OutWall>>,
    ball_query: Query<&Transform, With<ball::Ball>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ball_transform = ball_query.single();

    out_wall_query.iter().for_each(|out_wall_transform| {
        let collision = collide(
            out_wall_transform.translation,
            out_wall_transform.scale.truncate(),
            ball_transform.translation,
            ball_transform.scale.truncate(),
        );

        if let Some(_) = collision {
            next_state.set(AppState::GameOver);
            return;
        }
    })
}
