use bevy::{app::PluginGroupBuilder, prelude::*};

mod ball;
mod block;
mod block_collision;
mod blocks;
mod collider;
mod config;
mod position;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_startup_system(setup)
        .add_system(block::transform_position)
        .add_system(ball::transform_position)
        .run();
}

fn plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "ブロック崩し".into(),
                resolution: config::Screen::SIZE.into(),
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
        position::Position { x: 100., y: 100. },
    )
}
