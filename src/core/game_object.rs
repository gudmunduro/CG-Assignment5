use glow::*;
use sdl2::event::Event;

use super::game::Game;

#[derive(Clone)]
pub enum CollisionInfo {
    NoCollision,
    YCollision(f32),
    // minX, minY, minZ, maxX, maxY, maxZ
    BoxCollision(f32, f32, f32, f32, f32, f32),
    MultiCollision(Vec<CollisionInfo>),
}

pub trait GameObject<'a> {
    fn collision_info(&self) -> CollisionInfo {
        return CollisionInfo::NoCollision;
    }

    fn on_event(&mut self, game: &Game, event: &Event);
    fn update(&mut self, game: &Game, gl: &'a Context);
    fn display(&self, game: &Game, gl: &'a Context);
}