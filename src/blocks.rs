use bevy::prelude::*;

use crate::{block, config, position};

pub fn spawn(
    commands: &mut Commands,
    init_pos_y_from_top: f32,
    x_num: u32,
    y_num: u32,
    bundle: impl Bundle + Copy,
) {
    let init_pos_x = -(config::Block::SIZE.x + config::Block::GAP) * (x_num - 1) as f32 / 2.;
    let mut pos = position::Position {
        x: init_pos_x,
        y: config::Field::SIZE.y / 2. - init_pos_y_from_top,
    };

    for _ in 0..y_num {
        pos.x = init_pos_x;
        for _ in 0..x_num {
            block::spawn(commands, pos, bundle);
            pos.x += config::Block::SIZE.x + config::Block::GAP;
        }
        pos.y -= config::Block::SIZE.y + config::Block::GAP;
    }
}
