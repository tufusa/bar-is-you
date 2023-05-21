use bevy::prelude::*;

use crate::{app_state::AppState, font, in_game};

#[derive(Component)]
pub struct GameOver;

pub fn setup(mut commands: Commands, ui_font: Res<font::UI>) {
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
        .insert(GameOver)
        .with_children(|parent| spawn(parent, ui_font));
}

fn spawn(parent: &mut ChildBuilder, ui_font: Res<font::UI>) {
    parent.spawn(TextBundle::from_section(
        "Press space to retry",
        TextStyle {
            font: ui_font.clone().0,
            font_size: 50.,
            color: Color::WHITE,
        },
    ));
}

pub fn check_input(input: Res<Input<KeyCode>>, mut next_state: ResMut<NextState<AppState>>) {
    if input.pressed(KeyCode::Space) {
        next_state.set(AppState::InGame);
    }
}

pub fn cleanup(
    mut commands: Commands,
    in_game_query: Query<Entity, With<in_game::InGame>>,
    game_over_query: Query<Entity, With<GameOver>>,
) {
    in_game_query.iter().for_each(|in_game| {
        commands.entity(in_game).despawn_recursive();
    });
    game_over_query.iter().for_each(|game_over| {
        commands.entity(game_over).despawn_recursive();
    })
}
