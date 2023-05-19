use bevy::{app::PluginGroupBuilder, prelude::*};

mod block;
mod block_collision;
mod blocks;
mod config;
mod position;
mod collider;
mod ball;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_startup_system(setup)
        .add_system(block::transform_position)
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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    blocks::spawn(commands, position::Position { x: -100., y: -100. }, 5, 5);
}
