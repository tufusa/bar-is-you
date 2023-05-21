#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示
use bevy::{app::PluginGroupBuilder, prelude::*, window::*};

mod ball;
mod block;
mod blocks;
mod collider;
mod config;
mod position;
mod velocity;
mod wall;
mod wall_location;
mod walls;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_startup_system(setup)
        .add_system(block::transform_position)
        .add_system(block::collision_ball)
        .add_system(ball::transform_position)
        .add_system(ball::position_velocity)
        .add_system(ball::reflection_event_handler)
        .add_system(wall::collision_ball)
        .add_system(window_move_event_handler)
        .add_event::<ball::ReflectionEvent>()
        .run();
}

fn plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "ブロック崩し".into(),
                resolution: config::Screen::SIZE.into(),
                mode: WindowMode::Windowed,
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

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut window_query: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());
    blocks::spawn(
        &mut commands,
        position::Position { x: -100., y: -100. },
        5,
        5,
    );
    ball::spawn(
        &mut commands,
        meshes,
        materials,
        position::Position { x: -200., y: -200. },
        velocity::Velocity { x: 500., y: 500. },
    );
    walls::spawn(&mut commands);
    window_query.single_mut().position = WindowPosition::Centered(MonitorSelection::Current);
}

fn window_move_event_handler(mut window_move_event_reader: EventReader<WindowMoved>) {
    if let Some(event) = window_move_event_reader.iter().next() {
        println!("{:?}", event.position);
    }
}
