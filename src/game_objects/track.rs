use glow::{Context, NativeTexture};
use nalgebra::{Vector3, Vector2};

use crate::{core::{game_object::{GameObject, CollisionInfo}, game::Game, color::Color}, objects::textured_square::TexturedSquare, utils::FacingDirection};

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

        track.push(TrackSegment::new(SegmentType::Straight(Vector3::new(0.0, 0.0, 66.0), FacingDirection::North, 240.0), gl, game));
        track.push(TrackSegment::new(SegmentType::RightCorner(Vector3::new(-10.0, 0.0, 285.0)), gl, game));
        track.push(TrackSegment::new(SegmentType::Straight(Vector3::new(186.0, 0.0, 275.0), FacingDirection::West, 200.0), gl, game));
        // S
        track.push(TrackSegment::new(SegmentType::UCorner(Vector3::new(286.0, 0.0, 235.0), 90f32.to_radians()), gl, game));
        track.push(TrackSegment::new(SegmentType::UCorner(Vector3::new(292.3, 0.0, 159.0), 270f32.to_radians()), gl, game));
        track.push(TrackSegment::new(SegmentType::UCorner(Vector3::new(292.0, 0.0, 79.0), 90f32.to_radians()), gl, game));

        // Track end
        track.push(TrackSegment::new(SegmentType::Straight(Vector3::new(226.0, 0.0, 40.0), FacingDirection::West, 140.0), gl, game));
        track.push(TrackSegment::new(SegmentType::RightCorner(Vector3::new(67.0, 0.0, 50.0)), gl, game));
        track.push(TrackSegment::new(SegmentType::UCorner(Vector3::new(37.0, 0.0, -49.0), 180f32.to_radians()), gl, game));
        // track.push(RoadSurface::new(Vector3::new(105.0, 0.0, 200.0), Vector2::new(200.0, 10.0), gl, game));

        track
    }
}

impl<'a> GameObject<'a> for Track<'a> {

    fn collision_info(&self) -> CollisionInfo {
        CollisionInfo::MultiCollision(self.track.iter().map(|s| s.collision_info()).collect())
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
