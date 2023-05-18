use bevy::{app::PluginGroupBuilder, prelude::*};
mod block;
mod config;
mod position;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_startup_system(setup)
        .run();
}

fn plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "ブロック崩し".into(),
                resolution: (config::SCREEN_WIDTH as f32, config::SCREEN_HEIGHT as f32).into(),
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
    block::spawn(&mut commands, position::Position { x: 0, y: 0 })
}
