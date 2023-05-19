use bevy::prelude::*;

use crate::{block, config, position};

pub fn spawn(commands: &mut Commands, init_pos: position::Position, x_num: u32, y_num: u32) {
    let mut pos = init_pos;
    for _ in 0..y_num {
        pos.x = init_pos.x;
        for _ in 0..x_num {
            block::spawn(commands, pos);
            pos.x += config::Block::SIZE.x + config::Block::GAP;
        }
        pos.y += config::Block::SIZE.y + config::Block::GAP;
    }
}
