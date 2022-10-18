pub enum CollisionInfo {
    NoCollision
}

pub trait GameObject {
    pub fn collision_info() -> CollisionInfo {
        return CollisionInfo::NoCollision;
    }
}