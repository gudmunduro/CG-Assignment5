use glow::{Context, NativeTexture};
use nalgebra::{Vector2, Vector3};

use crate::{
    core::{
        color::Color,
        game::Game,
        game_object::GameObject,
    },
    objects::{track_corner::{TrackCorner, TrackCornerType}, textured_square::TexturedSquare}, utils::FacingDirection,
};

const TRACK_ELEVATION: f32 = 10.0;
const TRACK_STRAIGHT_LENGTH: f32 = 10.0;

pub enum SegmentType {
    // Position, direction, length
    Straight(Vector3<f32>, FacingDirection, f32),
    // Enter, control, exit, track width
    // Corner(Vector3<f32>, Vector3<f32>, Vector3<f32>, f32),
    // Position
    RightCorner(Vector3<f32>),
    UCorner(Vector3<f32>),
}

enum SegmentObject<'a> {
    Straight(TexturedSquare<'a>, Vector3<f32>, FacingDirection, f32),
    RightCorner(TrackCorner<'a>, Vector3<f32>),
    UCorner(TrackCorner<'a>, Vector3<f32>),
}

pub struct TrackSegment<'a> {
    segment_object: SegmentObject<'a>,
    road_texture: NativeTexture,
}

impl<'a> TrackSegment<'a> {
    pub fn new(segment_type: SegmentType, gl: &'a Context, game: &Game) -> TrackSegment<'a> {
        let road_texture = game.load_texture("./models/textures/road.png", true);

        use SegmentType::*;
        let segment_object = match segment_type {
            Straight(pos, direction, length) => {
                SegmentObject::Straight(TexturedSquare::new(gl, TRACK_STRAIGHT_LENGTH, length, FacingDirection::North), pos, direction, length)
            }
            RightCorner(pos) => {
                SegmentObject::RightCorner(TrackCorner::new(gl, TrackCornerType::Right), pos)
            }
            UCorner(pos) => {
                SegmentObject::UCorner(TrackCorner::new(gl, TrackCornerType::UTurn), pos)
            }
        };

        TrackSegment {
            road_texture,
            segment_object,
        }
    }
}

impl<'a> GameObject<'a> for TrackSegment<'a> {
    fn on_event(&mut self, game: &Game, event: &sdl2::event::Event) {}

    fn update(&mut self, game: &Game, gl: &'a Context) {}

    fn display(&self, game: &Game, gl: &'a Context) {
        let mut model_matrix = game.model_matrix.borrow_mut();

        game.shader.set_material_ambient(&Color::new(0.5, 0.5, 0.5));
        game.shader.set_material_diffuse(&Color::new(0.7, 0.7, 0.7));
        game.shader
            .set_material_specular(&Color::new(1.0, 1.0, 1.0));
        game.shader.set_shininess(3.0);

        use SegmentObject::*;
        match &self.segment_object {
            Straight(object, pos, dir, length) => {
                model_matrix.push_stack();

                use FacingDirection::*;
                match dir {
                    North => {
                        model_matrix.add_translate(pos.x, pos.y + TRACK_ELEVATION + 0.1, pos.z + pos.z * 0.09);
                        model_matrix.add_scale(1.0, 1.0, 1.09);
                    }
                    West => {
                        model_matrix.add_translate(pos.x + pos.x * 0.07, pos.y + TRACK_ELEVATION + 0.1, pos.z);
                        model_matrix.add_scale(1.11, 1.0, 1.0);
                        model_matrix.add_rotation(0.0, 90f32.to_radians(), 0.0);    
                    }
                }

                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
                model_matrix.pop_stack();

                game.shader.set_material_ambient(&Color::new(0.89 / 2.0, 0.62 / 2.0, 0.14 / 2.0));
                game.shader.set_material_diffuse(&Color::new(0.89, 0.62, 0.14));
                game.shader
                    .set_material_specular(&Color::new(1.0, 1.0, 1.0));
                game.shader.set_shininess(3.0);

                model_matrix.push_stack();
                model_matrix.add_translate(pos.x, pos.y + TRACK_ELEVATION / 2.0, pos.z);
                if matches!(dir, FacingDirection::West) {
                    model_matrix.add_rotation(0.0, 90f32.to_radians(), 0.0);
                }
                model_matrix.add_scale(TRACK_STRAIGHT_LENGTH + 5.0, TRACK_ELEVATION, *length);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();
            }
            /*Corner(object, enter, control, exit) => {
                let center = (0.5 * enter + 0.5 * exit) + 0.5 * control;
                let size = ((enter - exit).abs() - control).abs();
                
                model_matrix.push_stack();
                model_matrix.add_translate(0.0, TRACK_ELEVATION + 0.1, 0.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
                model_matrix.pop_stack();

                model_matrix.push_stack();
                model_matrix.add_translate(center.x, center.y + TRACK_ELEVATION / 2.0, center.z);
                model_matrix.add_scale(size.x, TRACK_ELEVATION, size.z);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();
            }*/
            RightCorner(object, pos) => {
                model_matrix.push_stack();
                model_matrix.add_translate(pos.x, TRACK_ELEVATION + 0.1, pos.z);
                model_matrix.add_scale(100.0, 1.0, 100.0);
                model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
                model_matrix.pop_stack();

                /*model_matrix.push_stack();
                model_matrix.add_translate(center.x, center.y + TRACK_ELEVATION / 2.0, center.z);
                model_matrix.add_scale(size.x, TRACK_ELEVATION, size.z);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();*/
            }
            UCorner(object, pos) => {
                model_matrix.push_stack();
                model_matrix.add_translate(pos.x, TRACK_ELEVATION + 0.1, pos.z);
                model_matrix.add_scale(100.0, 1.0, 100.0);
                model_matrix.add_rotation(0.0, 270f32.to_radians(), 0.0);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                object.draw(&game.shader, &self.road_texture);
                model_matrix.pop_stack();

                /*model_matrix.push_stack();
                model_matrix.add_translate(center.x, center.y + TRACK_ELEVATION / 2.0, center.z);
                model_matrix.add_scale(size.x, TRACK_ELEVATION, size.z);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();*/
            }
        }
    }
}
