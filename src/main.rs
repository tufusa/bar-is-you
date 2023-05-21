#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示
use app_state::AppState;
use bevy::{app::PluginGroupBuilder, prelude::*, window::*};
use bevy_embedded_assets::EmbeddedAssetPlugin; // asstesを実行ファイルに埋め込む

mod app_state;
mod ball;
mod bar;
mod block;
mod blocks;
mod collider;
mod config;
mod game;
mod position;
mod title;
mod velocity;
mod wall;
mod wall_location;
mod walls;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_state::<AppState>()
        .add_event::<ball::ReflectionEvent>()
        .add_system(setup.on_startup())
        .add_system(title::setup.in_schedule(OnEnter(AppState::Title)))
        .add_system(title::input.in_set(OnUpdate(AppState::Title)))
        .add_system(title::cleanup.in_schedule(OnExit(AppState::Title)))
        .add_system(game::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_systems(
            (
                block::transform_position,
                block::collision_ball,
                ball::transform_position,
                ball::transform_position,
                ball::position_velocity,
                ball::reflection_event_handler,
                wall::collision_ball,
                bar::transform_position,
                bar::collision_ball,
                bar::move_position,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .run();
}

fn plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: config::Title::TITLE.into(),
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
        .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
}
