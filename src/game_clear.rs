use bevy::prelude::*;

use crate::{app_state::AppState, font, in_game, ui};

#[derive(Component)]
pub struct GameClear;

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
                "Game Clear",
                TextStyle {
                    font: title_font.clone().0,
                    font_size: 150.,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect::top(Val::Px(150.)),
                ..Default::default()
            }),
        )
        .insert(GameClear);
    parent
        .spawn(
            TextBundle::from_section(
                "Press space to go back to title",
                TextStyle {
                    font: ui_font.clone().0,
                    font_size: 50.,
                    color: Color::WHITE,
                },
            )
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect::bottom(Val::Px(20.)),
                ..Default::default()
            }),
        )
        .insert(GameClear);
}

pub fn check_input(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::Space) {
        next_state.set(AppState::Title);
    }
}

pub fn cleanup(
    mut commands: Commands,
    in_game_query: Query<Entity, With<in_game::InGame>>,
    game_over_query: Query<Entity, With<GameClear>>,
) {
    in_game_query.iter().for_each(|in_game| {
        commands.entity(in_game).despawn_recursive();
    });
    game_over_query.iter().for_each(|game_over| {
        commands.entity(game_over).despawn_recursive();
    })
}
