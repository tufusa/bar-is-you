use bevy::prelude::*;

use crate::{app_state::AppState, config};

#[derive(Component)]
pub struct Title;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let amatic_bold: Handle<Font> = asset_server.load("fonts/AmaticSC-Bold.ttf");
    let roboto_thin: Handle<Font> = asset_server.load("fonts/Roboto-Thin.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(Title)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    config::Title::TITLE,
                    TextStyle {
                        font: amatic_bold,
                        font_size: 300.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Percent(15.),
                        bottom: Val::Auto,
                    },
                    ..Default::default()
                }),
            );
            parent.spawn(
                TextBundle::from_section(
                    "Press space to start",
                    TextStyle {
                        font: roboto_thin,
                        font_size: 50.,
                        color: Color::WHITE,
                    },
                )
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Percent(35.),
                        bottom: Val::Auto,
                    },
                    ..Default::default()
                }),
            );
        });
}

pub fn input(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn cleanup(mut commands: Commands, title_query: Query<Entity, With<Title>>) {
    title_query
        .iter()
        .for_each(|title| commands.entity(title).despawn_recursive());
}
