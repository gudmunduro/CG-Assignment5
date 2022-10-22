use glow::{Context, NativeTexture};
use nalgebra::{Vector3, Vector2};

use crate::{core::{game_object::{GameObject, CollisionInfo}, game::Game, color::Color}, objects::textured_cube::TexturedCube};

use super::road_surface::RoadSurface;


pub struct Track<'a> {
    surface_cube: TexturedCube<'a>,
    road_texture: NativeTexture,
    track: Vec<RoadSurface<'a>>
}

impl<'a> Track<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Track<'a> {
        let road_texture = game.load_texture("./models/textures/road.png");
        let surface_cube = TexturedCube::new(gl);
        let track = Track::create_track(gl, game);

        Track { surface_cube, road_texture, track }
    }

    fn create_track<'b>(gl: &'b Context, game: &Game) -> Vec<RoadSurface<'b>> {
        let mut track = Vec::new();


        for i in 0..20 {
            track.push(RoadSurface::new(Vector3::new(0.0, 0.0, 10.0 * i as f32), Vector2::new(10.0, 10.0), gl, game));    
        }

        for i in 0..20 {
            track.push(RoadSurface::new(Vector3::new(10.0 * i as f32, 0.0, 10.0 * 20.0), Vector2::new(10.0, 10.0), gl, game));    
        }

        track
    }
}

impl<'a> GameObject<'a> for Track<'a> {

    fn collision_info(&self) -> CollisionInfo {
        CollisionInfo::YCollision(0.0)
    }

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        for segment in &self.track {
            segment.display(game, gl);
        }
    }
}
