use glow::{Context, NativeTexture};
use nalgebra::{Vector2, Vector3};

use crate::{
    core::{
        color::Color,
        game::Game,
        game_object::{CollisionInfo, GameObject},
    },
    objects::{track_corner::TrackCorner, textured_square::TexturedSquare},
};

const TRACK_ELEVATION: f32 = 10.0;

pub enum SegmentType {
    // Position, size
    Straight(Vector3<f32>, Vector2<f32>),
    // Enter, control, exit, track width
    Corner(Vector3<f32>, Vector3<f32>, Vector3<f32>, f32),
}

enum SegmentObject<'a> {
    Straight(TexturedSquare<'a>, Vector3<f32>, Vector2<f32>),
    Corner(TrackCorner<'a>, Vector3<f32>, Vector3<f32>, Vector3<f32>),
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
            Straight(pos, size) => {
                SegmentObject::Straight(TexturedSquare::new(gl, size.x, size.y), pos, size)
            }
            Corner(enter, control, exit, width) => {
                SegmentObject::Corner(TrackCorner::new(gl, enter, control, exit, width), enter, control, exit)
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
            Straight(object, pos, size) => {
                model_matrix.push_stack();
                model_matrix.add_translate(pos.x, pos.y + TRACK_ELEVATION + 0.1, pos.z);
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
                model_matrix.add_scale(size.x + 5.0, TRACK_ELEVATION, size.y);
                game.shader.set_model_matrix(model_matrix.matrix.as_slice());
                game.cube.draw(&game.shader);
                model_matrix.pop_stack();
            }
            Corner(object, enter, control, exit) => {
                let center = (0.5 * enter + 0.5 * exit) + 0.5 * control;
                let size = ((enter - exit).abs() - control).abs();
                
                model_matrix.push_stack();
                model_matrix.add_translate(0.0, TRACK_ELEVATION, 0.0);
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
