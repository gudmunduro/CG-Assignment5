use glow::{Context, NativeTexture};
use nalgebra::Vector3;

use crate::{
    core::{
        color::Color,
        game::Game,
        game_object::{Collider, GameObject},
    },
    game_objects::{
        track_segment::{TRACK_BOX_HEIGHT, TRACK_ELEVATION, TRACK_WIDTH},
        track_side::{self, Side},
    },
    objects::textured_square::TexturedSquare,
    utils::FacingDirection,
};

use super::track_side::TrackSide;

pub struct TrackStraightSegment<'a> {
    segment_object: TexturedSquare<'a>,
    position: Vector3<f32>,
    direction: FacingDirection,
    length: f32,
    sides: TrackSide,
    road_texture: NativeTexture,
}

impl<'a> TrackStraightSegment<'a> {
    pub fn new(
        position: Vector3<f32>,
        direction: FacingDirection,
        length: f32,
        gl: &'a Context,
        game: &Game,
    ) -> TrackStraightSegment<'a> {
        let road_texture = game.load_texture("./models/textures/road.png", true);

        use FacingDirection::*;
        let rot = match direction {
            North => 0.0,
            West => 90f32.to_radians(),
        };

        let pos = position + Vector3::new(0.0, TRACK_ELEVATION + 0.5, 0.0);
        let sides = TrackSide::new(
            pos,
            rot,
            length,
            track_side::TrackSegmentSideType::Straight,
            game,
        );

        TrackStraightSegment {
            segment_object: TexturedSquare::new(gl, TRACK_WIDTH, length, FacingDirection::North),
            position,
            direction,
            length,
            sides,
            road_texture,
        }
    }
}

impl<'a> GameObject<'a> for TrackStraightSegment<'a> {
    fn collision_info(&self) -> crate::core::game_object::Collider {
        self.sides.collision_info()
    }

    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {}

    fn update(&mut self, game: &Game, gl: &'a Context) {}

    fn display(&self, game: &Game, gl: &'a Context) {
        self.sides.display(game, gl);

        let mut model_matrix = game.model_matrix.borrow_mut();

        game.shader.set_material_ambient(&Color::new(0.5, 0.5, 0.5));
        game.shader.set_material_diffuse(&Color::new(0.7, 0.7, 0.7));
        game.shader
            .set_material_specular(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_shininess(3.0);

        model_matrix.push_stack();

        use FacingDirection::*;
        match self.direction {
            North => {
                model_matrix.add_translate(
                    self.position.x,
                    self.position.y + TRACK_ELEVATION + 0.1,
                    self.position.z,
                );
                model_matrix.add_scale(1.0, 1.0, 1.0);
            }
            West => {
                model_matrix.add_translate(
                    self.position.x,
                    self.position.y + TRACK_ELEVATION + 0.1,
                    self.position.z,
                );
                model_matrix.add_scale(1.0, 1.0, 1.0);
                model_matrix.add_rotation(0.0, 90f32.to_radians(), 0.0);
            }
        }

        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        self.segment_object.draw(&game.shader, &self.road_texture);
        model_matrix.pop_stack();

        game.shader
            .set_material_ambient(&Color::new(0.89 / 2.0, 0.62 / 2.0, 0.14 / 2.0));
        game.shader
            .set_material_diffuse(&Color::new(0.89, 0.62, 0.14));
        game.shader
            .set_material_specular(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_shininess(3.0);

        model_matrix.push_stack();
        model_matrix.add_translate(
            self.position.x,
            self.position.y + TRACK_ELEVATION - (TRACK_BOX_HEIGHT / 2.0),
            self.position.z,
        );
        if matches!(self.direction, FacingDirection::West) {
            model_matrix.add_rotation(0.0, 90f32.to_radians(), 0.0);
        }
        model_matrix.add_scale(TRACK_WIDTH + 5.0, TRACK_BOX_HEIGHT, self.length);
        game.shader.set_model_matrix(model_matrix.matrix.as_slice());
        game.cube.draw(&game.shader);
        model_matrix.pop_stack();
    }
}
