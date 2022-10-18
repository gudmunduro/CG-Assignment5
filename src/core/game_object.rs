use glow::*;

use super::game::Game;

pub enum CollisionInfo {
    NoCollision
}

pub trait GameObject<'a> {
    fn collision_info(&self) -> CollisionInfo {
        return CollisionInfo::NoCollision;
    }

    fn update(&mut self, game: &Game, gl: &'a Context);
    fn display(&self, game: &Game, gl: &'a Context);
}