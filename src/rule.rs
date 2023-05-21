use bevy::prelude::*;

use crate::font;

#[derive(Component)]
struct Rule;

pub fn spawn(parent: &mut ChildBuilder, font: Res<font::Rule>, component: impl Component) {
    let style = &TextStyle {
        font: font.clone().0,
        font_size: 70.,
        color: Color::WHITE,
    };

    parent
        .spawn(
            TextBundle::from_sections([
                TextSection::new("Bar Is You\n", style.clone()),
                TextSection::new("Ball Is Move\n", style.clone()),
                TextSection::new("Bottom Is Death\n", style.clone()),
                TextSection::new("Breakall Is Win\n", style.clone()),
            ])
            .with_text_alignment(TextAlignment::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect::top(Val::Px(300.)),
                ..Default::default()
            }),
        )
        .insert(component);
}
