#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseではコンソールを非表示
use app_state::AppState;
use bevy::{app::PluginGroupBuilder, prelude::*, window::*};

#[cfg(not(target_family = "wasm"))]
use bevy_embedded_assets::EmbeddedAssetPlugin; // asstesを実行ファイルに埋め込む

mod app_state;
mod ball;
mod bar;
mod block;
mod blocks;
mod collider;
mod collision;
mod config;
mod field;
mod font;
mod game_clear;
mod game_over;
mod in_game;
mod out_wall;
mod position;
mod rule;
mod title;
mod ui;
mod velocity;
mod wall;
mod wall_location;

fn main() {
    App::new()
        .add_plugins(plugins())
        .add_state::<AppState>()
        .add_event::<ball::ReflectionEvent>()
        .add_event::<ball::JustifyEvent>()
        .add_system(setup.on_startup())
        .add_system(title::setup.in_schedule(OnEnter(AppState::Title)))
        .add_system(title::check_input.in_set(OnUpdate(AppState::Title)))
        .add_system(title::cleanup.in_schedule(OnExit(AppState::Title)))
        .add_system(in_game::setup.in_schedule(OnEnter(AppState::InGame)))
        .add_systems(
            (
                in_game::check_break_all,
                in_game::check_out_wall,
                field::check_rule,
                block::transform_position,
                block::collision_ball,
                ball::transform_position,
                ball::position_velocity,
                ball::input_position,
                ball::reflection_event_handler,
                ball::justify_event_handler,
                wall::collision_ball,
                bar::transform_position,
                bar::collision_ball,
                bar::input_position,
                rule::server_displayer,
            )
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_systems(
            (block::position_velocity, block::collision_wall).in_set(OnUpdate(AppState::InGame)),
        )
        .add_system(game_over::setup.in_schedule(OnEnter(AppState::GameOver)))
        .add_system(game_over::check_input.in_set(OnUpdate(AppState::GameOver)))
        .add_system(game_over::cleanup.in_schedule(OnExit(AppState::GameOver)))
        .add_system(game_clear::setup.in_schedule(OnEnter(AppState::GameClear)))
        .add_system(game_clear::check_input.in_set(OnUpdate(AppState::GameClear)))
        .add_system(game_clear::cleanup.in_schedule(OnExit(AppState::GameClear)))
        .run();
}

#[cfg(not(target_family = "wasm"))]
fn plugins() -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: config::Title::TITLE.into(),
                mode: WindowMode::Fullscreen,
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        })
        .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
}

#[cfg(target_family = "wasm")]
fn plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: config::Title::TITLE.into(),
            resolution: (1536., 864.).into(),
            ..Default::default()
        }),
        ..Default::default()
    })
}

fn setup(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    ui::spawn(&mut commands);
    commands.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
    commands.insert_resource(font::Title(server.load("fonts/AmaticSC-Bold.ttf")));
    commands.insert_resource(font::UI(server.load("fonts/Roboto-Thin.ttf")));
    commands.insert_resource(font::Rule(server.load("fonts/AmaticSC-Regular.ttf")));
}
