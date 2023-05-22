use bevy::prelude::*;

use crate::font;

#[derive(Component)]
pub struct RuleServer {
    pub rule: Rule,
}

#[derive(Component)]
pub struct RuleDisplayer;

pub struct Rule {
    pub is_you: IsYou,
    pub is_death: IsDeath,
    pub is_win: IsWin,
    pub is_move: IsMove,
}

#[derive(PartialEq, Eq)]
pub enum IsYou {
    None,
    Bar,
    Ball,
}

#[derive(PartialEq, Eq)]
pub enum IsDeath {
    None,
    Out,
    Block,
}

#[derive(PartialEq, Eq)]
pub enum IsWin {
    None,
    BreakAll,
}

#[derive(PartialEq, Eq)]
pub enum IsMove {
    None,
    Ball,
    Bar,
    Block,
}

impl RuleServer {
    fn from(rule: Rule) -> Self {
        Self { rule }
    }
}

impl IsYou {
    fn to_string(&self) -> Option<String> {
        match self {
            IsYou::None => Option::None,
            IsYou::Bar => Some("Bar Is You".into()),
            IsYou::Ball => Some("Ball Is You".into()),
        }
    }
}

impl IsDeath {
    fn to_string(&self) -> Option<String> {
        match self {
            IsDeath::None => Option::None,
            IsDeath::Out => Some("Out Is Death".into()),
            IsDeath::Block => Some("Block Is Death".into()),
        }
    }
}

impl IsWin {
    fn to_string(&self) -> Option<String> {
        match self {
            IsWin::None => Option::None,
            IsWin::BreakAll => Some("BreakAll Is Win".into()),
        }
    }
}

impl IsMove {
    fn to_string(&self) -> Option<String> {
        match self {
            IsMove::None => Option::None,
            IsMove::Ball => Some("Ball Is Move".into()),
            IsMove::Bar => Some("Bar Is Move".into()),
            IsMove::Block => Some("Block Is Move".into()),
        }
    }
}

pub fn spawn_server(commands: &mut Commands, rule: Rule, bundle: impl Bundle) {
    commands
        .spawn(RuleServer::from(rule))
        .insert(bundle);
}

pub fn spawn_displayer(
    parent: &mut ChildBuilder,
    font: Res<font::Rule>,
    bundle: impl Bundle,
) {
    let style = &TextStyle {
        font: font.clone().0,
        font_size: 70.,
        color: Color::WHITE,
    };

    let rule_texts = vec![
        TextSection::from_style(style.clone()),
        TextSection::from_style(style.clone()),
        TextSection::from_style(style.clone()),
        TextSection::from_style(style.clone()),
    ];

    parent
        .spawn(
            TextBundle::from_sections(rule_texts)
                .with_text_alignment(TextAlignment::Center)
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::top(Val::Px(320.)),
                    ..Default::default()
                }),
        )
        .insert(RuleDisplayer)
        .insert(bundle);
}

pub fn server_displayer(
    server_query: Query<&RuleServer>,
    mut displayer_query: Query<&mut Text, With<RuleDisplayer>>,
) {
    let server = server_query.single();
    let mut displayer = displayer_query.single_mut();

    displayer.sections[0].value = server.rule.is_you.to_string().unwrap_or("".into()) + "\n";
    displayer.sections[1].value = server.rule.is_move.to_string().unwrap_or("".into()) + "\n";
    displayer.sections[2].value = server.rule.is_death.to_string().unwrap_or("".into()) + "\n";
    displayer.sections[3].value = server.rule.is_win.to_string().unwrap_or("".into()) + "\n";
}
