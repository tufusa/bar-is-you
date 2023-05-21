use bevy::prelude::*;

use crate::{app_state::AppState, config, font, ui};

#[derive(Component)]
pub struct Title;

pub fn setup(
    mut commands: Commands,
    ui_base_query: Query<Entity, With<ui::Base>>,
    title_font: Res<font::Title>,
    ui_font: Res<font::UI>,
) {
    commands
        .entity(ui_base_query.single())
        .with_children(|parent| spawn(parent, title_font, ui_font));
}

fn spawn(parent: &mut ChildBuilder, title_font: Res<font::Title>, ui_font: Res<font::UI>) {
    parent
        .spawn(
            TextBundle::from_section(
                config::Title::TITLE,
                TextStyle {
                    font: title_font.to_owned().0,
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
        )
        .insert(Title);
    parent
        .spawn(
            TextBundle::from_section(
                "Press space to start",
                TextStyle {
                    font: ui_font.clone().0,
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
        )
        .insert(Title);
}

pub fn check_input(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn cleanup(mut commands: Commands, title_query: Query<Entity, With<Title>>) {
    title_query
        .iter()
        .for_each(|title| commands.entity(title).despawn_recursive());
}
