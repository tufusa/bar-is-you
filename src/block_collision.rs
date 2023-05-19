use bevy::{prelude::*, sprite::collide_aabb::*};

use crate::{block, collider};

pub struct CollisionEvent;

pub fn collision(
    mut commands: Commands,
    mut ball_query: Query<()>,
    block_query: Query<(Entity, &Transform, &block::Block), With<collider::Collider>>,
    mut collision_event_writer: EventWriter<CollisionEvent>,
) {
}
