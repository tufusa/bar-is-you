use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    app_state::AppState, ball, bar, block::Block, blocks, config, field, font, position, rule,
    rule_routine, ui, velocity,
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
    spawn(&mut commands, meshes, materials, window_query.single());

    rule::spawn_server(&mut commands, config::Rule::INIT, InGame);
    rule_routine::setup(&mut commands);

    commands
        .entity(ui_base_query.single())
        .with_children(|parent| rule::spawn_displayer(parent, rule_font, InGame));
}

fn spawn(
    commands: &mut Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    window: &Window,
) {
    blocks::spawn(commands, 50., 15, 5, InGame);
    ball::spawn(
        commands,
        meshes,
        materials,
        position::Position {
            x: 0.,
            y: -200. + config::Ball::SIZE.y,
        },
        velocity::Velocity { x: 220., y: 220. },
        InGame,
    );
    field::spawn(commands, InGame);
    bar::spawn(
        commands,
        position::Position { x: 0., y: -200. },
        velocity::Velocity { x: 200., y: 200. },
        InGame,
    );
}

pub fn check_break_all(
    block_query: Query<&Block>,
    mut next_state: ResMut<NextState<AppState>>,
    rule_server_query: Query<&rule::RuleServer>,
) {
    if rule_server_query.single().rule.is_win != rule::IsWin::BreakAll {
        return;
    }

    if block_query.is_empty() {
        next_state.set(AppState::GameClear);
    }
}

pub fn check_out(
    ball_query: Query<&Transform, With<ball::Ball>>,
    window_query: Query<&Window>,
    mut next_state: ResMut<NextState<AppState>>,
    rule_server_query: Query<&rule::RuleServer>,
) {
    if rule_server_query.single().rule.is_death != rule::IsDeath::Out {
        return;
    }

    let ball_transform = ball_query.single();
    let window = window_query.single();
    let window_size = Vec2::new(window.width(), window.height());

    let collision = collide(
        Vec3::ZERO,
        window_size,
        ball_transform.translation,
        ball_transform.scale.truncate(),
    );
    if collision.is_none() {
        next_state.set(AppState::GameOver);
    }
}
