use glow::Context;

use crate::{
    core::{
        game::Game,
        game_object::{CollisionInfo, GameObject},
    },
};

use super::{track_straight_segment::TrackStraightSegment, track_right_corner_segment::TrackRightCornerSegment, track_u_corner_segment::TrackUCornerSegment};

pub const TRACK_ELEVATION: f32 = 30.0;
pub const TRACK_BOX_HEIGHT: f32 = 5.0;
pub const TRACK_WIDTH: f32 = 20.0;

pub enum TrackSegment<'a> {
    // Position, direction, length
    Straight(TrackStraightSegment<'a>),
    // Position
    RightCorner(TrackRightCornerSegment<'a>),
    // Position, rotation
    UCorner(TrackUCornerSegment<'a>),
}

impl<'a> GameObject<'a> for TrackSegment<'a> {
    fn collision_info(&self) -> CollisionInfo {
        use TrackSegment::*;
        match self {
            Straight (s) => s.collision_info(),
            RightCorner (s) => s.collision_info(),
            UCorner (s) => s.collision_info(),
        }
    }

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {
        use TrackSegment::*;
        match self {
            Straight (s) => s.on_event(game, event),
            RightCorner (s) => s.on_event(game, event),
            UCorner (s) => s.on_event(game, event),
        }
    }

    fn update(&mut self, game: &Game, gl: &'a Context) {
        use TrackSegment::*;
        match self {
            Straight (s) => s.update(game, gl),
            RightCorner (s) => s.update(game, gl),
            UCorner (s) => s.update(game, gl),
        }
    }

    fn display(&self, game: &Game, gl: &'a Context) {
        use TrackSegment::*;
        match self {
            Straight (s) => s.display(game, gl),
            RightCorner (s) => s.display(game, gl),
            UCorner (s) => s.display(game, gl),
        }
    }
}