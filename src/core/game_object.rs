use glow::*;
use nalgebra::Vector3;
use sdl2::event::Event;

use super::game::Game;

#[derive(Clone)]
pub enum Collider {
    NoCollision,
    HeightCollider(f32),
    // minX, minY, minZ, maxX, maxY, maxZ
    BoxCollider(f32, f32, f32, f32, f32, f32),
    MultiCollider(Vec<Collider>),
    InfiniteYPlaneCollider(Vector3<f32>, Vector3<f32>),
}

pub trait GameObject<'a> {
    fn collision_info(&self) -> Collider {
        return Collider::NoCollision;
    }

    fn on_event(&mut self, game: &Game, event: &Event);
    fn update(&mut self, game: &Game, gl: &'a Context);
    fn display(&self, game: &Game, gl: &'a Context);
}