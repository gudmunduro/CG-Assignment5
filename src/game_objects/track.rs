use glow::{Context, NativeTexture};
use nalgebra::{Vector3, Vector2};

use crate::{core::{game_object::{GameObject, CollisionInfo}, game::Game, color::Color}, objects::textured_square::TexturedSquare};

use super::track_segment::{TrackSegment, SegmentType};


pub struct Track<'a> {
    track: Vec<TrackSegment<'a>>
}

impl<'a> Track<'a> {
    pub fn new(gl: &'a Context, game: &Game) -> Track<'a> {
        let track = Track::create_track(gl, game);

        Track { track }
    }

    fn create_track<'b>(gl: &'b Context, game: &Game) -> Vec<TrackSegment<'b>> {
        let mut track = Vec::new();

        track.push(TrackSegment::new(SegmentType::Straight(Vector3::new(0.0, 0.0, 105.0), Vector2::new(10.0, 200.0)), gl, game));
        track.push(TrackSegment::new(SegmentType::Corner(Vector3::new(-5.0, 0.0, 205.0), Vector3::new(50.0, 1.0, 300.0), Vector3::new(100.0, 1.0, 250.0), 10.0), gl, game));
        track.push(TrackSegment::new(SegmentType::Straight(Vector3::new(100.0, 0.0, 250.0), Vector2::new(50.0, 10.0)), gl, game));
        // track.push(RoadSurface::new(Vector3::new(105.0, 0.0, 200.0), Vector2::new(200.0, 10.0), gl, game));

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
