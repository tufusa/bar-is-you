use bevy::sprite::collide_aabb;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Collision {
    Left,
    Right,
    Top,
    Bottom,
    Inside,
}

impl Collision {
    pub fn from(collision: collide_aabb::Collision) -> Self {
        match collision {
            collide_aabb::Collision::Left => Self::Left,
            collide_aabb::Collision::Right => Self::Right,
            collide_aabb::Collision::Top => Self::Top,
            collide_aabb::Collision::Bottom => Self::Bottom,
            collide_aabb::Collision::Inside => Self::Inside,
        }
    }
}
