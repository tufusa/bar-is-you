use bevy::prelude::*;

use crate::{config, in_game, rule, wall, wall_location::WallLocation};

#[derive(Component)]
pub struct BottomWall;

pub fn spawn(commands: &mut Commands, bundle: impl Bundle + Copy) {
    let (size, thickness) = (config::Field::SIZE, config::Wall::THICKNESS);
    wall::spawn(commands, WallLocation::Left, size, thickness, bundle);
    wall::spawn(commands, WallLocation::Right, size, thickness, bundle);
    wall::spawn(commands, WallLocation::Top, size, thickness, bundle);
}

pub fn check_rule(
    mut commands: Commands,
    bottom_wall_query: Query<Entity, With<BottomWall>>,
    rule_server_query: Query<&rule::RuleServer>,
) {
    if rule_server_query.single().rule.is_move != rule::IsMove::Block {
        bottom_wall_query.iter().for_each(|bottom_wall| {
            commands.entity(bottom_wall).despawn();
        });
        return;
    }

    if bottom_wall_query.is_empty() {
        wall::spawn(
            &mut commands,
            WallLocation::Bottom,
            config::Field::SIZE,
            config::Wall::THICKNESS,
            in_game::InGame,
        );
    }
}
