use bevy::prelude::*;

use crate::rule;

#[derive(Resource)]
pub(crate) struct Routine(pub(crate) Timer);

pub(crate) fn setup(commands: &mut Commands) {
    commands.insert_resource(Routine(Timer::new(
        std::time::Duration::from_secs(10),
        TimerMode::Repeating,
    )));
}

pub(crate) fn tick(time: Res<Time>, mut routine: ResMut<Routine>) {
    routine.0.tick(time.delta());
}

pub(crate) fn routine(
    mut rule_server_query: Query<&mut rule::RuleServer>,
    routile: ResMut<Routine>,
) {
    let mut rule_server = rule_server_query.single_mut();
    if !routile.0.finished() {
        return;
    }

    rule_server.rule = rule::Rule::random();
}
