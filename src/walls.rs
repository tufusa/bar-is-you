use bevy::prelude::*;

use crate::{wall, wall_location::WallLocation};

pub fn spawn(parent: &mut ChildBuilder) {
    wall::spawn(parent, WallLocation::Left);
    wall::spawn(parent, WallLocation::Right);
    wall::spawn(parent, WallLocation::Top);
    // wall::spawn(commands, WallLocation::Bottom);
}
