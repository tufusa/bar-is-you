use bevy::prelude::*;

use crate::{wall, wall_location::WallLocation};

pub fn spawn(commands: &mut Commands) {
    wall::spawn(commands, WallLocation::Left);
    wall::spawn(commands, WallLocation::Right);
    wall::spawn(commands, WallLocation::Top);
    wall::spawn(commands, WallLocation::Bottom);
}
