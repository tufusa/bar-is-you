#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示
use app_state::AppState;
use bevy::{app::PluginGroupBuilder, prelude::*, window::*};

mod app_state;
mod ball;
mod bar;
mod block;
mod blocks;
mod collider;
mod config;
mod position;
mod velocity;
mod wall;
mod wall_location;
mod walls;
mod title;
mod game;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_state::<AppState>()
        .add_event::<ball::ReflectionEvent>()
        .add_system(setup.on_startup())
        .add_system(title::setup.in_schedule(OnEnter(AppState::Title)))
        .add_system(title::title.in_set(OnUpdate(AppState::Title)))
        .add_system(game::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_systems((
            block::transform_position,
            block::collision_ball,
            ball::transform_position,
            ball::transform_position,
            ball::position_velocity,
            ball::reflection_event_handler,
            wall::collision_ball,
            bar::transform_position,
            bar::collision_ball,
            bar::move_position
        ).in_set(OnUpdate(AppState::InGame)))
        // .add_system(block::collision_ball)
        // .add_system(ball::transform_position)
        // .add_system(ball::position_velocity)
        // .add_system(ball::reflection_event_handler)
        // .add_system(wall::collision_ball)
        // .add_system(bar::transform_position)
        // .add_system(bar::collision_ball)
        // .add_system(bar::move_position)
        .run();
}

fn plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "ブロック崩し".into(),
                mode: WindowMode::BorderlessFullscreen,
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(AssetPlugin {
            watch_for_changes: true,
            ..Default::default()
        })
}

fn setup(mut commands: Commands, mut window_query: Query<&mut Window>) {
    commands.spawn(Camera2dBundle::default());
    window_query.single_mut().position = WindowPosition::Centered(MonitorSelection::Current);
}

// fn setup_game(
    // mut commands: Commands,
    // meshes: ResMut<Assets<Mesh>>,
    // materials: ResMut<Assets<ColorMaterial>>,
// ) {
    // blocks::spawn(&mut commands, 50., 15, 5);
    // ball::spawn(
        // &mut commands,
        // meshes,
        // materials,
        // position::Position {
            // x: 0.,
            // y: -200. + config::Ball::SIZE.y,
        // },
        // velocity::Velocity { x: 220., y: 220. },
    // );
    // walls::spawn(&mut commands);
    // bar::spawn(
        // &mut commands,
        // position::Position { x: 0., y: -200. },
        // velocity::Velocity { x: 200., y: 200. },
    // );
// }
// 