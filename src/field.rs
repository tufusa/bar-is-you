use bevy::prelude::*;

use crate::{config, wall, wall_location::WallLocation};

pub fn spawn(parent: &mut ChildBuilder, bundle: impl Bundle + Copy) {
    let (size, thickness) = (config::Field::SIZE, config::Wall::THICKNESS);
    wall::spawn(parent, WallLocation::Left, size, thickness, bundle);
    wall::spawn(parent, WallLocation::Right, size, thickness, bundle);
    wall::spawn(parent, WallLocation::Top, size, thickness, bundle);
    // wall::spawn(commands, WallLocation::Bottom);
}
