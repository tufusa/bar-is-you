use bevy::prelude::*;

#[derive(Component)]
pub struct Base;

pub fn spawn(commands: &mut Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Flex,
                ..default()
            },
            ..default()
        })
        .insert(Base);
}
